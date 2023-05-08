use colored::Colorize;
use std::io::Write;
use std::fs::File;

use crate::{config::TIME_FORMAT, timestamp};

pub enum LogLvl {
    Activity,
    Error,
    Warning,
}

impl ToString for LogLvl {
    fn to_string(&self) -> String {
        match self {
            Self::Activity => "[ACTIVITY]".yellow(),
            Self::Error => "[ERROR]".red(),
            Self::Warning => "[WARNING]".bright_yellow(),
        }.to_string()
    }
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
pub fn report(log: &mut File, msg: &str, print_output: bool, severity: LogLvl) {
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
#[cfg(target_os = "windows")]
#[macro_export]
macro_rules! pause_exit {
    () => {
        print!("press any key to exit . . .");
        std::io::stdout().flush().unwrap();
        let _ = std::process::Command::new("cmd.exe").args(["/c", "pause > nul"]).status();
        std::process::exit(1);
    };
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
#[macro_export]
macro_rules! pause_exit {
    () => {
            let _ = std::process::Command::new("/usr/bin/read").args(["-n 1", "-p press any key to exit . . ."]).status();
            std::process::exit(1);
    };
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
