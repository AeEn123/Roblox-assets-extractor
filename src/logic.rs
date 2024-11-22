use std::{fs, sync::Arc};
use std::collections::HashMap;
use std::io::Read;
use std::thread;
use std::sync::Mutex;
use fluent_bundle::{FluentBundle, FluentResource, FluentArgs};
use unic_langid::LanguageIdentifier;
use lazy_static::lazy_static;
use serde_json::{json, Value};

include!(concat!(env!("OUT_DIR"), "/locale_data.rs")); // defines get_locale_resources and LANGUAGE_LIST

// Define mutable static values
lazy_static! {
    static ref LANGUAGE_LIST: Mutex<HashMap<String,String>> = Mutex::new(init_language_list());

    static ref CONFIG: Mutex<Value> = Mutex::new(read_config_file());

    static ref TEMP_DIRECTORY: Mutex<Option<tempfile::TempDir>> = Mutex::new(None);
    static ref CACHE_DIRECTORY: Mutex<String> = Mutex::new(detect_directory());
    static ref STATUS: Mutex<String> = Mutex::new(get_message(&get_locale(None), "idling", None));
    static ref FILE_LIST: Mutex<Vec<String>> = Mutex::new(Vec::new());
    static ref REQUEST_REPAINT: Mutex<bool> = Mutex::new(false);
    static ref PROGRESS: Mutex<f32> = Mutex::new(1.0);

    
    static ref LIST_TASK_RUNNING: Mutex<bool> = Mutex::new(false);
    static ref STOP_LIST_RUNNING: Mutex<bool> = Mutex::new(false);
    static ref TASK_RUNNING: Mutex<bool> = Mutex::new(false); // Delete/extract


    // File headers for each catagory
    static ref HEADERS: Mutex<HashMap<String,[String;2]>> = {
        let mut m = HashMap::new();
        m.insert("sounds".to_owned(),[
            "OggS".to_owned(),
            "ID3".to_owned()
            ]);
        m.insert("images".to_owned(), [
            "PNG".to_owned(),
            "WEBP".to_owned()
            ]);
        m.insert("ktx-files".to_owned(), [
            "KTX".to_owned(),
            "".to_owned()
            ]);
        m.insert("rbxm-files".to_owned(), [
            "<roblox!".to_owned(),
            "".to_owned()
            ]);
        Mutex::new(m)
    };

    // File extention for headers
    static ref EXTENTION: Mutex<HashMap<String, String>> = {
        let mut m = HashMap::new();
        m.insert("OggS".to_owned(), ".ogg".to_owned());
        m.insert("ID3".to_owned(), ".mp3".to_owned());
        m.insert("PNG".to_owned(), ".png".to_owned());
        m.insert("WEBP".to_owned(), ".webp".to_owned());
        m.insert("KTX".to_owned(), ".ktx".to_owned());
        m.insert("<roblox!".to_owned(), ".rbxm".to_owned());
        Mutex::new(m)
    };

    // Header offsets, headers that are not in this HashMap not be offset
    // Offset will subtract from the found header.
    static ref OFFSET: Mutex<HashMap<String, usize>> = {
        let mut m = HashMap::new();
        m.insert("PNG".to_owned(), 1);
        m.insert("KTX".to_owned(), 1);
        m.insert("WEBP".to_owned(), 8);
        Mutex::new(m)
    };
}


const DEFAULT_DIRECTORIES: [&str; 2] = ["%Temp%\\Roblox", "~/.var/app/org.vinegarhq.Sober/cache/sober"]; // For windows and linux (sober)
const CONFIG_FILE: &str = "Roblox-assets-extractor-config.json";

// Define local functions
fn read_config_file() -> Value {
    match fs::read(CONFIG_FILE) {
        Ok(bytes) => {
            match serde_json::from_slice(&bytes) {
                Ok(v) => return v,
                Err(e) => {
                    eprintln!("Failed to parse config file! {}", e);
                    return json!({}); // Blank config by default
                }
            }
        }

        Err(_e) => {
            // Most likely no such file or directory
            return json!({});
        }
    }
}

fn init_language_list() -> HashMap<String,String> {
    let mut languages = LOCALES.to_vec();

    // Move the default language to the top of the language list
    let default_language = &sys_locale::get_locale().unwrap_or_else(|| "en-GB".to_string());
    if let Some(pos) = languages.iter().position(|lang| lang == &default_language) {
        let default_lang = languages.remove(pos);
        languages.insert(0, default_lang);
    }

    let mut m = HashMap::new();
    for lang in languages {
        m.insert(lang.to_owned(), get_message(&get_locale(Some(lang)), "language-name", None));
    }
    return m
    
}

fn update_status(value: String) {
    let mut status = STATUS.lock().unwrap();
    *status = value;
    let mut request = REQUEST_REPAINT.lock().unwrap();
    *request = true;
}

fn update_progress(value: f32) {
    let mut progress = PROGRESS.lock().unwrap();
    *progress = value;
    let mut request = REQUEST_REPAINT.lock().unwrap();
    *request = true;
}

fn update_file_list(value: String, cli_list_mode: bool) {
    // cli_list_mode will print out to console
    // It is done this way so it can read files and print to console in the same stage
    if cli_list_mode {
        println!("{}", value);
    }
    let mut file_list = FILE_LIST.lock().unwrap();
    file_list.push(value)
}

fn clear_file_list() {
    let mut file_list = FILE_LIST.lock().unwrap();
    *file_list = Vec::new()
}

fn bytes_search(haystack: Vec<u8>, needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

fn bytes_contains(haystack: &[u8], needle: &[u8]) -> bool {
    haystack.windows(needle.len()).any(|window| window == needle)
}

fn find_header(mode: String, bytes: Vec<u8>) -> String {
    // Get headers and offsets, they will be used later
    let all_headers = {
        HEADERS.lock().unwrap().clone()
    };

    // Get the header for the current mode
    let option_headers = all_headers.get(&mode);

    if let Some(headers) = option_headers {
        // Itearte through headers to find the correct one for this file.
        for header in headers {
            if bytes_contains(&bytes, header.as_bytes()) {
                return header.to_owned()
            }
        }
    }
    return "INVALID".to_owned()
}

fn extract_bytes(header: String, bytes: Vec<u8>) -> Vec<u8> {
    // Get offsets for headers
    let offsets = {
        OFFSET.lock().unwrap().clone()
    };

    // Find the header in the file
    if let Some(mut index) = bytes_search(bytes.clone(), header.as_bytes()) {
        // Found the header, extract from the bytes
        if let Some(offset) = offsets.get(&header) {
            // Apply offset to index if the offset exists
            index -= *offset;
        }
        // Return all the bytes after the found header index
        return bytes[index..].to_vec()
    }
    println!("WARN: Failed to extract a file!");
    // Return bytes instead if this fails
    return bytes
}

// Define public functions
pub fn validate_directory(directory: &str) -> Result<String, String> {
    let resolved_directory = directory
    .replace("%Temp%", &format!("C:\\Users\\{}\\AppData\\Local\\Temp", whoami::username()))
    .replace("~", &format!("/home/{}", whoami::username()));
    // There's probably a better way of doing this... It works though :D

    match fs::metadata(&resolved_directory) { // Directory detection
        Ok(metadata) => {
            if metadata.is_dir() {
                // Successfully detected a directory, we can return it
                return Ok(resolved_directory);
            } else {
                return Err(format!("{}: Not a directory", resolved_directory));
            }
        }
        Err(e) => {
            return Err(e.to_string()); // Convert to correct data type
        }
    }
}

pub fn detect_directory() -> String {
    let mut errors = "".to_owned();
    if let Some(directory) = get_config().get("cache_directory") {
        // User-specified directory from config
        match validate_directory(&directory.to_string().replace('"',"")) { // It kept returning "value" instead of value
            Ok(resolved_directory) => return resolved_directory,
            Err(e) => {
                println!("User-defined directory is invalid: {}", e);
                errors.push_str(&e.to_string());
            },
        }
    }
    // Directory detection
    for directory in DEFAULT_DIRECTORIES {
        match validate_directory(directory) {
            Ok(resolved_directory) => return resolved_directory,
            Err(e) => errors.push_str(&e.to_string()),
        }  

    }

    // If it was unable to detect any directory, tell the user and panic the program
    let _ = native_dialog::MessageDialog::new()
    .set_type(native_dialog::MessageType::Error)
    .set_title(&get_message(&get_locale(None), "error-directory-detection-title", None))
    .set_text(&get_message(&get_locale(None), "error-directory-detection-description", None))
    .show_alert();
    panic!("Directory detection failed!{}", errors);

}

pub fn get_locale(lang: Option<&str>) -> FluentBundle<Arc<FluentResource>> {
    let locale = if let Some(locale) = lang {
        locale
    } else {
        // If language is not provided, get language from config
        if let Some(language) = get_config_string("language") {
            &language.clone()
        } else {
            // The language is not in the config file.
            &sys_locale::get_locale().unwrap_or_else(|| "en-GB".to_string()) // If locale cannot be identified, default to English
        }
        
        
    };
    
    let resource_data = if let Some(resources) = get_locale_resources(&locale) {
        resources
    } else {
        get_locale_resources("en-GB").unwrap() // Use English if the locale is not supported
    };

    let resource = FluentResource::try_new(resource_data).expect("Failed to parse FTL string.");
    
    let lang_id: LanguageIdentifier = locale.parse().unwrap_or_else(|_| "en-GB".parse().unwrap());
    let mut bundle = FluentBundle::new(vec![lang_id]);

    bundle.add_resource_overriding(resource.into());
    bundle
}

pub fn get_message(locale: &FluentBundle<Arc<FluentResource>>, id: &str, args: Option<& FluentArgs<'_>>) -> String {
    if let Some(message) = locale.get_message(id) {
        if let Some(value) = message.value() {
            let mut err = vec![];
            return locale.format_pattern(value, args, &mut err).to_string();
        } else {
            return id.to_owned() // Return id if it is not available
        }
    } else {
        return id.to_owned(); // Return id if it is not available
    }
}

// Function to get temp directory, create it if it doesn't exist
pub fn get_temp_dir(create_directory: bool) -> String {
    let mut option_temp_dir = TEMP_DIRECTORY.lock().unwrap();
    if let Some(temp_dir) = option_temp_dir.as_ref() {
        return temp_dir.path().to_string_lossy().to_string();
    } else if create_directory  {
        match tempfile::tempdir() {
            Ok(temp_dir) => {
                let path = temp_dir.path().to_string_lossy().to_string();
                *option_temp_dir = Some(temp_dir);
                return path;
            }
            Err(e) => {
                // Have a visual dialog to show the user what actually went wrong
                let _ = native_dialog::MessageDialog::new()
                .set_type(native_dialog::MessageType::Error)
                .set_title(&get_message(&get_locale(None), "error-temporary-directory-title", None))
                .set_text(&get_message(&get_locale(None), "error-temporary-directory-description", None))
                .show_alert();
                panic!("Failed to create a temporary directory! {}", e)
            }
        }
    } else {
        return "".to_string();
    }
}


pub fn delete_all_directory_contents(dir: String) {
    if dir == "" {
        panic!("Panic!ed due to safety. cache_directory was blank! Can possibly DELETE EVERYTHING!")
    }
    // Bunch of error checking to check if it's a valid directory
    match fs::metadata(dir.clone()) {
        Ok(metadata) => {
            if metadata.is_dir() {
                let running = {
                    let task = TASK_RUNNING.lock().unwrap();
                    task.clone()
                };
                // Stop multiple threads from running
                if running == false {
                    thread::spawn(|| {
                        { 
                            let mut task = TASK_RUNNING.lock().unwrap();
                            *task = true; // Stop other threads from running
                        }
                        // Get locale for localised status messages
                        let locale = get_locale(None);
                        
                        // Read directory
                        let entries: Vec<_> = fs::read_dir(dir).unwrap().collect();

                        // Get amount and initlilize counter for progress
                        let total = entries.len();
                        let mut count = 0;

                        for entry in entries {
                            count += 1; // Increase counter for progress
                            update_progress(count as f32/total as f32); // Convert to f32 to allow floating point output
                            let path = entry.unwrap().path();

                            // Args for formatting
                            let mut args = FluentArgs::new();
                            args.set("item", count);
                            args.set("total", total);
                            if path.is_dir() {
                                match fs::remove_dir_all(path) {
                                    // Error handling and update status
                                    Ok(_) => update_status(get_message(&locale, "deleting-files", Some(&args))),

                                    // If it's an error, log it and show on GUI
                                    Err(e) => {
                                        println!("ERROR: Failed to delete file: {}: {}", count, e);
                                        update_status(get_message(&locale, "failed-deleting-file", Some(&args)));
                                    }
                                }
                            } else {
                                match fs::remove_file(path) {
                                    // Error handling and update status
                                    Ok(_) => update_status(get_message(&locale, "deleting-files", Some(&args))),
    
                                    // If it's an error, log it and show on GUI
                                    Err(e) => {
                                        println!("ERROR: Failed to delete file: {}: {}", count, e);
                                        update_status(get_message(&locale, "failed-deleting-file", Some(&args)));
                                    }
                                }    
                            }
                        
                            
                        }
                        // Clear the file list for visual feedback to the user that the files are actually deleted
                        clear_file_list();
                        update_file_list(get_message(&locale, "no-files", None), false);
                        { 
                            let mut task = TASK_RUNNING.lock().unwrap();
                            *task = false; // Allow other threads to run again
                        }
                        update_status(get_message(&locale, "idling", None)); // Set the status back
                    });
                }
            // Error handling just so the program doesn't crash for seemingly no reason
            } else {
                update_status(get_message(&get_locale(None), "error-check-logs", None)); 
                println!("ERROR: Directory detection failed.")
            }
        }
        Err(e) => {
            println!("WARN: '{}' {}", dir, e);
            update_status(get_message(&get_locale(None), "idling", None)); 
        }
    }
}

pub fn refresh(dir: String, mode: String, cli_list_mode: bool, yield_for_thread: bool) {
    // Bunch of error checking to check if it's a valid directory
    match fs::metadata(dir.clone()) {
        Ok(metadata) => {
            if metadata.is_dir() {
                
                let handle = thread::spawn(move || {
                    // Get locale for localised status messages
                    let locale = get_locale(None);
                    // This loop here is to make it wait until it is not running, and to set the STOP_LIST_RUNNING to true if it is running to make the other thread
                    loop {
                        let running = {
                            let task = LIST_TASK_RUNNING.lock().unwrap();
                            task.clone()
                        };
                        if !running {
                            break // Break if not running
                        } else {
                            let mut stop = STOP_LIST_RUNNING.lock().unwrap(); // Tell the other thread to stop
                            *stop = true;
                        }
                        thread::sleep(std::time::Duration::from_millis(10)); // Sleep for a bit to not be CPU intensive
                    }
                    { 
                        let mut task = LIST_TASK_RUNNING.lock().unwrap();
                        *task = true; // Tell other threads that a task is running
                        let mut stop = STOP_LIST_RUNNING.lock().unwrap();
                        *stop = false; // Disable the stop, otherwise this thread will stop!
                    }

                    clear_file_list(); // Only list the files on the current tab

                    // Read directory
                    let entries: Vec<_> = fs::read_dir(dir).unwrap().collect();

                    // Get amount and initlilize counter for progress
                    let total = entries.len();
                    let mut count = 0;

                    // Tell the user that there is no files to list to make it easy to tell that the program is working and it isn't broken
                    if total == 0 {
                        update_file_list(get_message(&locale, "no-files", None).to_owned(), cli_list_mode);
                    }

                    if mode != "music" { // Music lists files directly and others filter.
                        // Filter the files out
                        let all_headers = {
                            HEADERS.lock().unwrap().clone()
                        };
                        
                        let headers = if let Some(value) = all_headers.get(&mode) {
                            value
                        } else {
                            return
                        };

                        for entry in entries {
                            let stop = {
                                let stop_task = STOP_LIST_RUNNING.lock().unwrap();
                                stop_task.clone()
                            };
                            if stop {
                                break // Stop if another thread requests to stop this task.
                            }
                            
                            count += 1; // Increase counter for progress
                            update_progress(count as f32/total as f32); // Convert to f32 to allow floating point output
                            let path = entry.unwrap().path();
                            let display = path.display();

                            // Args for formatting
                            let mut args = FluentArgs::new();
                            args.set("item", count);
                            args.set("total", total);
    
                            if let Some(filename) = path.file_name() {
                                match &mut fs::File::open(&path) {
                                    Err(why) => {
                                        println!("ERROR: couldn't open {}: {}", display, why);
                                        args.set("error", why.to_string());
                                        update_status(get_message(&locale, "failed-opening-file", Some(&args)));
                                    },
                                    Ok(file) => {
                                        // Reading the first 2048 bytes
                                        let mut buffer = vec![0; 2048];
                                        match file.read(&mut buffer) {
                                            Err(why) => {
                                                println!("ERROR: couldn't open {}: {}", display, why);
                                                update_status(get_message(&locale, "failed-opening-file", Some(&args)));
                                            },
                                            Ok(bytes_read) => {
                                                buffer.truncate(bytes_read);
                                                for header in headers {
                                                    // Check if header is empty before actually checking file
                                                    if header != "" {
                                                        // Add the file if the file contains the header
                                                        if bytes_contains(&buffer, header.as_bytes()) {
                                                            update_file_list(filename.to_string_lossy().to_string(), cli_list_mode);
                                                        }
                                                    }
      
                                                }

                                                update_status(get_message(&locale, "reading-files", Some(&args)));
                                            }
                                        }
                                        
                                    },
                                };
                            }
                        }
                    } else {
                        // List the files from the directory instead of filtering
                        for entry in entries {
                            let stop = {
                                let stop_task = STOP_LIST_RUNNING.lock().unwrap();
                                stop_task.clone()
                            };
                            if stop {
                                break // Stop if another thread requests to stop this task.
                            }
                            
                            count += 1; // Increase counter for progress
                            update_progress(count as f32/total as f32);
                            let path = entry.unwrap().path();
                            if let Some(filename) = path.file_name() {
                                update_file_list(filename.to_string_lossy().to_string(), cli_list_mode);
                                update_status(format!("Reading files ({count}/{total})"));
                            }
                            
                        }
                    }


                    { 
                        let mut task = LIST_TASK_RUNNING.lock().unwrap();
                        *task = false; // Allow other threads to run again
                    }
                    update_status(get_message(&locale, "idling", None)); // Set the status back
                });

                if yield_for_thread {
                    // Will wait for the thread instead of quitting immediately
                    let _ = handle.join();
                }
            // Error handling just so the program doesn't crash for seemingly no reason
            } else {
                let mut status = STATUS.lock().unwrap();
                *status = format!("Error: check logs for more details.");
                println!("ERROR: Directory detection failed.")
            }
        }
        Err(e) => {
            println!("WARN: '{}' {}", dir, e);
            clear_file_list();
            update_file_list("No files to list.".to_owned(), cli_list_mode);
            update_status(get_message(&get_locale(None), "idling", None));
        }
    }
}

pub fn extract_file(file: String, mode: String, destination: String, add_extention: bool) -> String {
    match fs::metadata(file.clone()) {
        Ok(metadata) => {
            if metadata.is_file() {
                // This can return an error result
                let bytes_error = fs::read(file);
                match bytes_error {
                    // Remove the error result so the extract_bytes function can read it
                    Ok(bytes) => {
                        let header = find_header(mode, bytes.clone());
                        let extracted_bytes = if header != "INVALID" {
                            extract_bytes(header.clone(), bytes.clone())
                        } else {
                            bytes.clone()
                        };

                        let mut new_destination = destination.clone();

                        // Add the extention if needed
                        if add_extention {
                            let extentions = {EXTENTION.lock().unwrap().clone()};
                            if let Some(extention) = extentions.get(&header.clone()) {
                                new_destination = destination.clone() + &extention.clone()
                            } else {
                                new_destination = destination.clone() + ".ogg" // Music tab
                            }
                        }

                        // Ignore result, errors won't cause any further issues
                        let _ = fs::write(new_destination.clone(), extracted_bytes);
                        return new_destination;


                    }
                    Err(e) => {
                        update_status(get_message(&get_locale(None), "failed-opening-file", None));
                        println!("ERROR: Failed to open file: {}", e);
                        return "None".to_string();
                    }
                }
            // Error handling just so the program doesn't crash for seemingly no reason
            } else {
                // Args for formatting
                let mut args = FluentArgs::new();
                args.set("file", &file);

                update_status(get_message(&get_locale(None), "failed-not-file", Some(&args)));
                println!("ERROR: '{}' Not a file.", file);
                return "None".to_string();
            }
        }
        Err(e) => {
            // Args for formatting
            let mut args = FluentArgs::new();
            args.set("error", e.to_string());

            println!("Error extracting file: '{}' {}", file, e);
            update_status(get_message(&get_locale(None), "idling", Some(&args)));
            return "None".to_string();
        }
    }
}

pub fn extract_dir(dir: String, destination: String, mode: String, file_list: Vec<String>, yield_for_thread: bool) {
    // Bunch of error checking to check if it's a valid directory
    match fs::metadata(dir.clone()) {
        Ok(metadata) => {
            if metadata.is_dir() {
                let running = {
                    let task = TASK_RUNNING.lock().unwrap();
                    task.clone()
                };
                // Stop multiple threads from running
                if running == false {
                    let handle = thread::spawn(move || {
                        { 
                            let mut task = TASK_RUNNING.lock().unwrap();
                            *task = true; // Stop other threads from running
                        }

                        // Get locale for localised status messages
                        let locale = get_locale(None);

                        // Get amount and initlilize counter for progress
                        let total = file_list.len();
                        let mut count = 0;

                        for entry in file_list {
                            count += 1; // Increase counter for progress
                            update_progress(count as f32/total as f32); // Convert to f32 to allow floating point output
                            let origin = format!("{}/{}", dir, entry);
                            let dest = format!("{}/{}", destination, entry); // Local variable destination

                            // Args for formatting
                            let mut args = FluentArgs::new();
                            args.set("item", count);
                            args.set("total", total);

                            let result = extract_file(origin, mode.clone(), dest, true);
                            if result == "None" {
                                update_status(get_message(&locale, "failed-extracting-file", Some(&args)));
                            } else {
                                update_status(get_message(&locale, "extracting-files", Some(&args)));
                            }
                        
                            
                        }
                        { 
                            let mut task = TASK_RUNNING.lock().unwrap();
                            *task = false; // Allow other threads to run again
                        }
                        update_status(get_message(&locale, "all-extracted", None)); // Set the status to confirm to the user that all has finished
                    });
                    
                    if yield_for_thread {
                        // Will wait for the thread instead of quitting immediately
                        let _ = handle.join();
                    }
                }
            // Error handling just so the program doesn't crash for seemingly no reason
            } else {
                update_status(get_message(&get_locale(None), "error-check-logs", None)); 
                println!("ERROR: Directory detection failed.")
            }
        }
        Err(e) => {
            println!("WARN: '{}' {}", dir, e);
            update_status(get_message(&get_locale(None), "idling", None));
        }
    }
}

pub fn extract_all(destination: String, yield_for_thread: bool) {
    let running = {
        let task = TASK_RUNNING.lock().unwrap();
        task.clone()
    };
    // Stop multiple threads from running
    if running == false {
        let handle = thread::spawn(move || {
            { 
                let mut task = TASK_RUNNING.lock().unwrap();
                *task = true; // Stop other threads from running
            }

            // Get locale for localised status messages
            let locale = get_locale(None);

            let headers = {HEADERS.lock().unwrap().clone()};

            let mut all_headers: Vec<(String, String)> = Vec::new();

            for key in headers.keys() {
                if let Some(mode_headers) = headers.get(key) {
                    for single_header in mode_headers {
                        all_headers.push((single_header.to_string(), key.to_string()));
                    }
                }
            }

            let cache_directory = get_cache_directory();
            let music_directory = format!("{}/sounds", cache_directory);
            let http_directory = format!("{}/http", cache_directory);

            // Attempt to create directories
            let _ = fs::create_dir(destination.clone());
            let _ = fs::create_dir(format!("{}/Music", destination.clone()));
            
            // Loop through all types and create directories for them
            for key in headers.keys() {
                let _ = fs::create_dir(format!("{}/{}", destination.clone(), key));
            }

            // Stage 1: Read and extract music directory
            let entries: Vec<_> = fs::read_dir(music_directory.clone()).unwrap().collect();

            // Get amount and initlilize counter for progress
            let total = entries.len();
            let mut count = 0;
            for entry in entries {                            
                count += 1; // Increase counter for progress
                update_progress((count as f32/total as f32)/ 3.0);

                // Args for formatting
                let mut args = FluentArgs::new();
                args.set("item", count);
                args.set("total", total);

                let path = entry.unwrap().path();
                if let Some(filename) = path.file_name() {
                    let origin = format!("{}/{}", music_directory.clone(), filename.to_string_lossy().to_string());
                    let dest = format!("{}/Music/{}", destination, filename.to_string_lossy().to_string()); // Local destination
                    extract_file(origin, "Music".to_string(), dest, true);

                    // More formatting to show "Stage 1/3: Extracting files"
                    args.set("status", get_message(&locale, "extracting-files", Some(&args)));
                    args.set("stage", "1");
                    args.set("max", "3");

                    update_status(get_message(&locale, "stage", Some(&args)));
                }
            }

            // Stage 2: Filter the files
            let entries: Vec<_> = fs::read_dir(http_directory.clone()).unwrap().collect();

            // Initilize the Vec for the filtered files to go in
            let mut filtered_files: Vec<(String, String)> = Vec::new();

            // Get amount and initlilize counter for progress
            let total = entries.len();
            let mut count = 0;
            for entry in entries {                            
                count += 1; // Increase counter for progress
                update_progress(((count as f32/total as f32) +1.0) /3.0); // 2nd stage, will fill up the bar from 1/3 to 2/3

                // Args for formatting
                let mut args = FluentArgs::new();
                args.set("item", count);
                args.set("total", total);

                let path = entry.unwrap().path();
                if let Some(filename) = path.file_name() {
                    match &mut fs::File::open(&path) {
                        Err(why) => {
                            println!("ERROR: couldn't open file: {}", why);
                            update_status(get_message(&locale, "failed-opening-file", Some(&args)));
                        },
                        Ok(file) => {
                            // Reading the first 2048 bytes
                            let mut buffer = vec![0; 2048];
                            match file.read(&mut buffer) {
                                Err(why) => {
                                    println!("ERROR: couldn't open file: {}", why);
                                    update_status(get_message(&locale, "failed-opening-file", Some(&args)));
                                },
                                Ok(bytes_read) => {
                                    buffer.truncate(bytes_read);
                                    // header.0 = header, header.1 = mode
                                    for header in all_headers.clone() {
                                        // Check if header is not empty before actually checking file
                                        if header.0 != "" {
                                            // Add it to the list if the header is inside of the file.
                                            if bytes_contains(&buffer, header.0.as_bytes()) {                                        
                                                filtered_files.push((filename.to_string_lossy().to_string(), header.1))
                                            }
                                        }

                                    }

                                    // More formatting to show "Stage 2/3: Filtering files"
                                    args.set("status", get_message(&locale, "filtering-files", Some(&args)));
                                    args.set("stage", "2");
                                    args.set("max", "3");

                                    update_status(get_message(&locale, "stage", Some(&args)));
                                }
                            }
                            
                        },
                    };
                }
            }

            // Stage 3: Extract the files

            // Get amount and initlilize counter for progress
            let total = filtered_files.len();
            let mut count = 0;
            for file in filtered_files {
                count += 1; // Increase counter for progress
                update_progress(((count as f32/total as f32) +2.0) /3.0); // 3rd stage, will fill up the bar from 2/3 to 3/3

                // Args for formatting
                let mut args = FluentArgs::new();
                args.set("item", count);
                args.set("total", total);

                let origin = format!("{}/{}", http_directory.clone(), file.0);
                let dest = format!("{}/{}/{}", destination, file.1, file.0); // Local destination, stores in (destination/type/name)
                extract_file(origin, file.1, dest, true);

                // More formatting to show "Stage 3/3: Extracting files"
                args.set("status", get_message(&locale, "extracting-files", Some(&args)));
                args.set("stage", "3");
                args.set("max", "3");

                update_status(get_message(&locale, "stage", Some(&args)));
            }

            { 
                let mut task = TASK_RUNNING.lock().unwrap();
                *task = false; // Allow other threads to run again
            }
            update_status(get_message(&locale, "all-extracted", None)); // Set the status to confirm to the user that all has finished
        });
        
        if yield_for_thread {
            // Will wait for the thread instead of quitting immediately
            let _ = handle.join();
        }
    }
}

pub fn get_file_list() -> Vec<String> {
    FILE_LIST.lock().unwrap().clone()
}

pub fn get_cache_directory() -> String {
    CACHE_DIRECTORY.lock().unwrap().clone()
}

pub fn set_cache_directory(value: String) {
    let mut cache_directory = CACHE_DIRECTORY.lock().unwrap();
    *cache_directory = value;
}

pub fn get_status() -> String {
    STATUS.lock().unwrap().clone()
}

pub fn get_progress() -> f32 {
    PROGRESS.lock().unwrap().clone()
}

pub fn get_list_task_running() -> bool {
    LIST_TASK_RUNNING.lock().unwrap().clone()
}

pub fn get_config() -> Value {
    CONFIG.lock().unwrap().clone()
}

pub fn get_language_list() -> HashMap<String,String> {
    LANGUAGE_LIST.lock().unwrap().clone()
}

pub fn get_config_string(key: &str) -> Option<String> {
    if let Some(value) = get_config().get(key) {
        return Some(value.as_str()?.to_owned().replace('"',""));
    } else {
        return None;
    }
   
}

pub fn get_config_bool(key: &str) -> Option<bool> {
    if let Some(value) = get_config().get(key) {
        return Some(value.as_bool()?);
    } else {
        return None;
    }
   
}

pub fn set_config(value: Value) {
    let mut config = CONFIG.lock().unwrap();
    if *config != value {
        match serde_json::to_vec_pretty(&value) {
            Ok(data) => {
                let result = fs::write(CONFIG_FILE, data);
                if result.is_err() {
                    println!("Failed to write config file: {:?}", result)
                }
            },
            Err(e) => {
                println!("Failed to write config file: {}", e);
            }
        }
        
        *config = value;
    }

}

pub fn set_config_string(key: &str, value: &str) {
    let mut config = get_config();
    config[key] = value.into();
    set_config(config);
}

pub fn set_config_bool(key: &str, value: bool) {
    let mut config = get_config();
    config[key] = value.into();
    set_config(config);
}

pub fn get_request_repaint() -> bool {
    let mut request_repaint = REQUEST_REPAINT.lock().unwrap();
    let old_request_repaint = *request_repaint;
    *request_repaint = false; // Set to false when this function is called to acknoledge
    return old_request_repaint
}

// Delete the temp directory
pub fn clean_up() {
    let temp_dir = get_temp_dir(false);
    // Just in case if it somehow resolves to "/"
    if temp_dir != "" && temp_dir != "/" {
        println!("Cleaning up {}", temp_dir);
        let _ = fs::remove_dir_all(temp_dir); // Not too important, ignore value, and the last thing the program will run
    }
}