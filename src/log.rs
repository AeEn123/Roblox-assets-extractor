use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref LOG: Mutex<String> = Mutex::new(String::new());
}

fn log(log_type: &str, message: &str) {
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let log_message = format!("{}  {}{}", now, log_type, message);
    
    println!("{}", log_message);

    let mut log = LOG.lock().unwrap();

    log.push_str(&format!("{}\n", log_message));
}

pub fn info(message: &str) {
    log("INFO:  ", message)
}

pub fn warn(message: &str) {
    log("WARN:  ", message)
}

pub fn error(message: &str) {
    log("ERROR: ", message)
}

pub fn get_logs() -> String {
    return LOG.lock().unwrap().clone();
}