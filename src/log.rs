use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::locale;

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

pub fn critical_error(message: &str) {
    log("CRITICAL: ", message);

    let _ = native_dialog::MessageDialog::new()
    .set_type(native_dialog::MessageType::Error)
    .set_title(&locale::get_message(&locale::get_locale(None), "generic-error-critical", None))
    .set_text(message)
    .show_alert();
}

pub fn get_logs() -> String {
    return LOG.lock().unwrap().clone();
}

pub fn get_anonymous_logs() -> String {
    let logs = LOG.lock().unwrap().clone();
    // Remove all possible information
    let logs = logs.replace(&whoami::username(), "username");
    let logs = logs.replace(&whoami::realname(), "Real Name");
    let logs = logs.replace(&whoami::devicename(), "devicename");
    logs
}