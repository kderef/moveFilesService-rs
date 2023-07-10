use crate::TIME_FORMAT;
use colored::Colorize;

#[macro_export]
macro_rules! chrono_time {
    ($format:expr) => {
        chrono::Local::now().format($format).to_string()
    };
}

pub fn err(msg: String) {
    let time = chrono_time!(TIME_FORMAT);
    
    let _ = 
        msgbox::create("MoveFiles Error", format!("ERROR: {msg}").as_str(), msgbox::IconType::Error);

    eprintln!("[{}] {}: {msg}", time.green(), "[ERROR]".red());
}
pub fn wrn(msg: String) {
    let time = chrono_time!(TIME_FORMAT);
    
    let _ = 
        msgbox::create("MoveFiles Warning", format!("WARNING: {msg}").as_str(), msgbox::IconType::Error);

    eprintln!("[{}] {}: {msg}", time.green(), "[WARNING]".yellow());
}
pub fn info(msg: String) {
    let time = chrono_time!(TIME_FORMAT);

    println!("[{}] {}: {msg}", time.green(), "[INFO]".bright_white());
}