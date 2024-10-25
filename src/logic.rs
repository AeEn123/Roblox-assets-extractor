use std::fs;
use std::thread;
use std::sync::Mutex;
use std::time::Duration;
use lazy_static::lazy_static;

lazy_static! {
    static ref CACHE_DIRECTORY: Mutex<String> = Mutex::new(String::new());
    static ref STATUS: Mutex<String> = Mutex::new("Idling".to_owned());
    static ref REQUEST_REPAINT: Mutex<bool> = Mutex::new(false);
    static ref TASK_RUNNING: Mutex<bool> = Mutex::new(false);
    static ref STOP_RUNNING: Mutex<bool> = Mutex::new(false);
}


const DEFAULT_DIRECTORIES: [&str; 2] = ["%Temp%\\Roblox", "~/.var/app/org.vinegarhq.Sober/cache/sober"];
// For windows and linux (sober)

pub fn detect_directory() {
    // Directory detection
    for directory in DEFAULT_DIRECTORIES {
        let resolved_directory = directory
        .replace("%Temp%", &format!("C:\\Users\\{}\\AppData\\Local\\Temp", whoami::username()))
        .replace("~", &format!("/home/{}", whoami::username()));
        // There's probably a better way of doing this... It works though :D

        match fs::metadata(&resolved_directory) { // Directory detection
            Ok(metadata) => {
                if metadata.is_dir() {
                    let mut cache_dir = CACHE_DIRECTORY.lock().unwrap();
                    *cache_dir = resolved_directory;
                    break;
                }
            }
            Err(e) => {
                println!("WARN: {directory}: {e}");
            }
        }

    }
}

pub fn delete_all_directory_contents(dir: String) {
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
                            *task = true;
                        }
                        for entry in fs::read_dir(dir).unwrap() {
                            println!("{}", entry.unwrap().path().display());
                        }
                        for i in 1..10000 {
                            // Update status and request refresh
                            {
                                let mut status = STATUS.lock().unwrap();
                                *status = format!("Deleting files ({i}/10000)");
                                let mut request = REQUEST_REPAINT.lock().unwrap();
                                *request = true;
                            }
                            thread::sleep(Duration::from_millis(1));
                        }
                        { 
                            let mut task = TASK_RUNNING.lock().unwrap();
                            *task = false;
                        }
                    });
                }
            // Error handling just so the program doesn't crash for seemingly no reason
            } else {
                let mut status = STATUS.lock().unwrap();
                *status = format!("Error: check logs for more details.");
                println!("ERROR: Directory detection failed.")
            }
        }
        Err(e) => {
            let mut status = STATUS.lock().unwrap();
            *status = format!("Error: '{dir}' is not a valid directory: {e}");
            println!("ERROR: Directory detection failed. {}", format!("'{dir}' is not a valid directory: {e}"))
        }
    }
}

pub fn get_cache_directory() -> String {
    CACHE_DIRECTORY.lock().unwrap().clone()
}

pub fn get_status() -> String {
    STATUS.lock().unwrap().clone()
}

// pub fn set_status(value: String) {
//     let mut status = STATUS.lock().unwrap();
//     *status = value
// }

pub fn get_request_repaint() -> bool {
    let mut request_repaint = REQUEST_REPAINT.lock().unwrap();
    *request_repaint = false;
    return !*request_repaint
}

// pub fn set_request_repaint(value: bool) {
//     let mut request_repaint = REQUEST_REPAINT.lock().unwrap();
//     *request_repaint = value
// }

pub fn double_click(value: usize) {
    let mut status = STATUS.lock().unwrap();
    *status = format!("Double clicked {}", value);
}