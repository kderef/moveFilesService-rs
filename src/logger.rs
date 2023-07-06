use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{PathBuf, Path};

use chrono::Datelike;
use colored::Colorize;

use crate::pause_exit;

#[macro_export]
macro_rules! chrono_time {
    ($format:expr) => {
        chrono::Local::now().format($format).to_string()
    };
}

macro_rules! day {
    () => {
        chrono::Local::now().day()
    };
}

macro_rules! fd_err_new {
    ($err_log_name:expr, $time:expr) => {
        OpenOptions::new()
            .append(true)
            .create(true)
            .write(true)
            .open(&$err_log_name)
            .unwrap_or_else(|e| {
                eprintln!(
                    "[{}] {}: failed to open error log \"{}\" because of error: {e}",
                    $time.green(),
                    "[ERROR]".red(),
                    $err_log_name
                );

                pause_exit!();
            })
    };
}

macro_rules! fd_info_new {
    ($info_log_name:expr, $time:expr, $err_log:expr) => {
        OpenOptions::new()
            .append(true)
            .create(true)
            .write(true)
            .open(&$info_log_name)
            .unwrap_or_else(|e| {
                eprintln!(
                    "[{}] {}: failed to open info log \"{}\" because of error: {e}",
                    $time.green(),
                    "[ERROR]".red(),
                    $info_log_name
                );

                writeln!(
                    $err_log,
                    "[{}] [ERROR]: failed to open info log \"{}\" because of error: {e}",
                    $time, $info_log_name
                )
                .unwrap();

                pause_exit!();
            })
    };
}

pub fn get_log_path() -> PathBuf {
    Path::new(&simple_home_dir::home_dir().unwrap()).join(r"AppData\Local\MoveFilesService\logs")
}

type S = &'static str;

pub struct Logger {
    time_format: S,
    log_extension: S,
    info_log_prefix: S,
    err_log_prefix: S,
    info_log_name: String,
    err_log_name: String,
    err_fd: File,
    info_fd: File,
    day: u32,
}

impl Logger {
    pub fn new(time_format: S, log_extension: S, info_log_prefix: S, err_log_prefix: S) -> Self {
        let log_dir = get_log_path();

        let path_time = chrono_time!("%d-%m-%Y");
        let info_log_path = Path::new(&log_dir).join(format!("{info_log_prefix} - [{path_time}].{log_extension}"));
        let err_log_path = Path::new(&log_dir).join(format!("{err_log_prefix} - [{path_time}].{log_extension}"));
        let time = chrono_time!(time_format);

        let mut fd_error = fd_err_new!(err_log_path.clone().into_os_string().into_string().unwrap(), time);

        let fd_info = fd_info_new!(info_log_path.clone().into_os_string().into_string().unwrap(), time, fd_error);

        let d = day!();

        Self {
            time_format,
            log_extension,
            info_log_prefix,
            err_log_prefix,
            info_log_name: info_log_path.clone().into_os_string().into_string().unwrap(),
            err_log_name: err_log_path.clone().into_os_string().into_string().unwrap(),
            err_fd: fd_error,
            info_fd: fd_info,
            day: d,
        }
    }
    fn validate_logs(&mut self) {
        let time = chrono_time!(self.time_format);
        let cur_day = day!();

        if cur_day != self.day {
            self.day = cur_day;
            let chr_time = chrono_time!("%d-%m-%Y");

            self.err_log_name = format!(
                "{} - [{}].{}",
                self.err_log_prefix, chr_time, self.log_extension
            );
            self.info_log_name = format!(
                "{} - [{}].{}",
                self.info_log_prefix, chr_time, self.log_extension
            );
            self.err_fd = fd_err_new!(self.err_log_name, time);
            self.info_fd = fd_info_new!(self.err_log_name, time, self.err_fd);
        }
    }
    pub fn warn(&mut self, msg: &str) {
        self.validate_logs();

        let time = chrono_time!(self.time_format);

        let _ =
            msgbox::create("MoveFiles Warning", format!("WARNING: {msg}").as_str(), msgbox::IconType::Error);

        eprintln!("[{}] {}: {msg}", time.green(), "[WARNING]".yellow());
        writeln!(self.err_fd, "[{time}] [WARNING]: {msg}").unwrap();
    }
    pub fn info(&mut self, msg: &str) {
        self.validate_logs();

        let time = chrono_time!(self.time_format);

        println!("[{}] {}: {msg}", time.green(), "[INFO]".bold());
        writeln!(self.info_fd, "[{time}] [INFO]: {msg}").unwrap();
    }
    pub fn err(&mut self, msg: &str) {
        self.validate_logs();

        let time = chrono_time!(self.time_format);
    
        let _ = 
            msgbox::create("MoveFiles Error", format!("ERROR: {msg}").as_str(), msgbox::IconType::Error);

        eprintln!("[{}] {}: {msg}", time.green(), "[ERROR]".red());
        writeln!(&mut self.err_fd, "[{time}] [ERROR]: {msg}").unwrap();
    }
}
