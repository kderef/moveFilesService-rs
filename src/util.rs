use colored::Colorize;
use std::io::Write;

use crate::config::TIME_FORMAT;

pub enum LogLvl {
    Activity,
    Error,
}

impl ToString for LogLvl {
    fn to_string(&self) -> String {
        match &self {
            Self::Activity => "[ACTIVITY]".yellow().to_string(),
            Self::Error => "[ERROR]".red().to_string(),
        }
    }
}

/// get hour from 0 - 23
#[macro_export]
macro_rules! current_hour {
    () => {
        chrono::Local::now().hour()
    };
}

/// get current day 1 - 31
#[macro_export]
macro_rules! current_day {
    () => {
        chrono::Local::now().day()
    };
}

/// timestamp used for logging purposes
#[macro_export]
macro_rules! timestamp {
    () => {
        chrono::Local::now().format(TIME_FORMAT)
    };
    ($time_fmt:expr) => {
        chrono::Local::now().format($time_fmt)
    };
}

/// sleep for `$sleep_time` seconds, continuously printing how much time is left.
#[macro_export]
macro_rules! sleep_countdown {
    ($sleep_time:expr) => {
        for i in (1..=($sleep_time)).rev() {
            print!(
                "completed, sleeping for {}                          \r",
                format!("{}s", i).green()
            );
            std::io::stdout().flush().unwrap();
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    };
}

/// report the given msg to the log file `log`, and to stdout IF print_output is set to `true`
pub fn report(log: &mut std::fs::File, msg: &str, print_output: bool, severity: LogLvl) {
    let time = timestamp!();
    write!(log, "{} {}", time, msg).unwrap();
    if print_output {
        println!(
            "{} {} {}",
            time.to_string().green(),
            severity.to_string(),
            msg
        )
    }
}

/// if parameter $msg is given, print that message and run CMD.exe /c pause
///
/// if no parameters are given, pause with the default message.
#[macro_export]
macro_rules! pause {
    () => {
        let _ = std::process::Command::new("cmd.exe").args(["/c", "pause"]).status()
    };
    ($msg:expr) => {
        print!("{}", $msg);
        std::io::stdout().flush().unwrap();
        let _ = std::process::Command::new("cmd.exe").args(["/c", "pause > nul"]).status();
    }
}


/// macro to (re-)open error log
#[macro_export]
macro_rules! open_error_log {
    ($err_log_path:expr) => {
        OpenOptions::new()
            .append(true)
            .write(true)
            .create(true)
            .open($err_log_path)
            .unwrap_or_else(|e| {
                let time = timestamp!();
                eprintln!(
                    "{} {}: `{}`",
                    time.to_string().green(),
                    "ERROR in error_log::OpenOptions()".red(),
                    e.to_string().yellow()
                );

                exit(1);
        })
 
    };
}

/// macro to (re-)open activity log
#[macro_export]
macro_rules! open_activity_log {
    ($activ_log_path:expr, $err_log:expr) => {
        OpenOptions::new()
            .append(true)
            .write(true)
            .create(true)
            .open($activ_log_path)
            .unwrap_or_else(|e| {
                let time = timestamp!();
                eprintln!(
                    "{} {}: `{}`",
                    time.to_string().green(),
                    "ERROR in activity_log::OpenOptions()".red(),
                    e.to_string().yellow()
                );
                report(
                    &mut $err_log,
                    format!("in activity_log::OpenOptions(): `{e}`\n").as_str(),
                    false,
                    LogLvl::Error,
                );

                exit(1);
            })
    };
}