#![allow(non_snake_case)]

extern crate fs_extra;
extern crate walkdir;

mod config;
mod movefiles;
mod util;

use toml::Table;
use chrono::Datelike;
use colored::Colorize;
use config::*;
use fs_extra::{dir::move_dir, file::move_file};
use movefiles::*;
use std::path::MAIN_SEPARATOR;
use std::process::exit;
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
};
use util::{report, LogLvl};

fn main() {
    println!(
        "{LOGO} [{}{}]\n{SEPARATOR_LONG}",
        "v".yellow(),
        VERSION.yellow()
    );
    println!(
        "{}: {}\n",
        "start date".yellow(),
        timestamp!().to_string().green()
    );

    /* log files */
    let mut error_log: File;
    let mut activity_log: File;

    let mut start_date = current_day!();
    let mut error_log_path = timestamped_path(ERROR_LOG_PATH);
    let mut activity_log_path = timestamped_path(ACTIVITY_LOG_PATH);

    error_log = open_error_log!(error_log_path.clone());
    activity_log = open_activity_log!(activity_log_path.clone(), error_log);

    // check for config files (source.txt, destination) //

    let mut source;       // source directory
    let mut destination;  // destination directory
    let sleep_time;   // amount of time to sleep after loop{} iteration

    let file_copy_options = fs_extra::file::CopyOptions::new().overwrite(true);
    let dir_copy_options = fs_extra::dir::CopyOptions::new().overwrite(true);

    if !file_exists!(CONFIG_PATH) {
        report(&mut error_log, format!("the config file `{CONFIG_PATH}` does not exist, creating it with default config...").as_str(), true, LogLvl::Warning);
        let conf_file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(CONFIG_PATH);

        match conf_file {
            Ok(mut fd) => match write!(&mut fd, "{}", DEFAULT_CONFIG) {
                Ok(_) => {
                    report(&mut error_log, "the config file has been created. Please enter your values into it and restart the program.", true, LogLvl::Error);
                    pause!("press any key to exit . . .");
                    exit(1);
                }
                Err(msg) => {
                    report(&mut error_log, format!("failed to write the default config to the config file because of error: {}", msg.to_string()).as_str(), true, LogLvl::Error);
                    pause!("press any key to exit . . .");
                    exit(1);
                }
            },
            Err(msg) => {
                report(&mut error_log, format!("failed to open config file `{CONFIG_PATH}` for writing because of error: {}", msg.to_string()).as_str(), true, LogLvl::Error);
                pause!("press any key to exit . . .");
                exit(1);
            }
        }
    }
    let config_contents = std::fs::read_to_string(CONFIG_PATH).unwrap();
    if config_contents.trim().is_empty() {
        report(&mut error_log, format!("the config file `{CONFIG_PATH}` is empty, filling it with default config...").as_str(), true, LogLvl::Warning);
        let conf_file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(CONFIG_PATH);

        match conf_file {
            Ok(mut fd) => {
                match write!(&mut fd, "{}", DEFAULT_CONFIG) {
                    Ok(_) => {
                        report(&mut error_log, format!("please enter your values into the config file `{CONFIG_PATH}` and restart the program.").as_str(), true, LogLvl::Warning);
                        pause!("press any key to exit . . .");
                        exit(1);
                    },
                    Err(msg) => {
                        report(&mut error_log, format!("failed to open config file `{CONFIG_PATH}` for writing because of error: {}", msg.to_string()).as_str(), true, LogLvl::Warning);
                        pause!("press any key to exit . . .");
                        exit(1);
                    }
                }
            },
            Err(msg) => {
                report(&mut error_log, format!("failed to open config file `{CONFIG_PATH}` because of error: {}", msg.to_string()).as_str(), true, LogLvl::Error);
                pause!("press any key to exit . . .");
                exit(1);
            }
        }
    }

    /* read with TOML */
    let mut error_log_ref = &mut error_log;
    let parsed_contents = config_contents.parse::<Table>().unwrap_or_else(move |e| {
        report(&mut error_log_ref, format!("failed to parse config, because of error: {}", e.to_string()).as_str(), true, LogLvl::Error);
        pause!("press any key to exit . . .");
        exit(1);
    });

    let source_val = parsed_contents.get("source");
    let destination_val = parsed_contents.get("destination");
    let seconds_val = parsed_contents.get("seconds");

    let mut error_encountered = false;

    if source_val.is_none() {
        report(&mut error_log, format!("in config file `{CONFIG_PATH}`: \"source\" is not set. Set it with source = \"path\".").as_str(), true, LogLvl::Error);
        error_encountered = true;
    }
    if destination_val.is_none() {
        report(&mut error_log, format!("in config file `{CONFIG_PATH}`: \"destination\" is not set. Set it with destination = \"path\".").as_str(), true, LogLvl::Error);
        error_encountered = true;
    }
    if seconds_val.is_none() {
        report(&mut error_log, format!("in config file `{CONFIG_PATH}`: \"seconds\" is not set. Set it with seconds = amount of seconds (example: seconds = 5)").as_str(), true, LogLvl::Error);
        error_encountered = true;
    }

    if !source_val.unwrap().is_str() {
        report(&mut error_log, format!("in config file `{CONFIG_PATH}`: key \"source\" should be a string, in the form {}.", "source = \"path\"".yellow()).as_str(), true, LogLvl::Error);
        error_encountered = true;
    }
    if !destination_val.unwrap().is_str() {
        report(&mut error_log, format!("in config file `{CONFIG_PATH}`: key \"destination\" should be a string, in the form {}.", "destination = \"path\"".yellow()).as_str(), true, LogLvl::Error);
        error_encountered = true;
    }
    if !seconds_val.unwrap().is_integer() {
        report(&mut error_log, format!("in config file `{CONFIG_PATH}`: key \"seconds\" should be a whole number, in the form {}.", "seconds = number".yellow()).as_str(), true, LogLvl::Error);
        error_encountered = true;
    }

    sleep_time = seconds_val.unwrap().as_integer().unwrap();
    if sleep_time < 0 || sleep_time > (config::SECONDS_MAX as i64) {
        report(&mut error_log, format!("in file `{CONFIG_PATH}`: key \"seconds\" should be > 0s and < {}s ({}h)",
            config::SECONDS_MAX, config::SECONDS_MAX / 3600).as_str(),
            true,
            LogLvl::Error
        );
        error_encountered = true;
    }

    if error_encountered {
        pause!("press any key to exit . . .");
        exit(1);
    }

    source = source_val.unwrap().as_str().unwrap().to_owned();
    destination = destination_val.unwrap().as_str().unwrap().to_owned();

    if !source.ends_with(MAIN_SEPARATOR) && !source.trim().is_empty() {
        source.push(MAIN_SEPARATOR);
    }
    if !destination.ends_with(MAIN_SEPARATOR) && !destination.trim().is_empty() {
        destination.push(MAIN_SEPARATOR);
    }

    if !dir_exists!(&source) {
        report(&mut error_log, format!("in config file `{CONFIG_PATH}`: the source directory '{source}' does not exist.").as_str(), true, LogLvl::Error);
        pause!("press any key to exit . . .");
        exit(1);
    }
    if !dir_exists!(&destination) {
        report(&mut error_log, format!("in config file `{CONFIG_PATH}`: the destination directory '{destination}' does not exist.").as_str(), true, LogLvl::Error);
        pause!("press any key to exit . . .");
        exit(1);
    }

    println!("{}:\t\t{}", "source dir".yellow(), source);
    println!("{}:\t{}", "destination dir".yellow(), destination);
    println!("{SEPARATOR}");

    loop {
        if current_day!() != start_date {
            start_date = current_day!();
            error_log_path = timestamped_path(ERROR_LOG_PATH);
            activity_log_path = timestamped_path(ACTIVITY_LOG_PATH);
            // if is a new day, reopen logs.
            error_log = open_error_log!(error_log_path.clone());
            activity_log = open_activity_log!(activity_log_path.clone(), error_log);
        }

        if !file_exists!(&error_log_path.clone()) {
            error_log = open_error_log!(error_log_path.clone());
        }
        if !file_exists!(&activity_log_path.clone()) {
            activity_log = open_activity_log!(activity_log_path.clone(), error_log);
        }

        for item in walkdir::WalkDir::new(&source).min_depth(1) {
            match item {
                Ok(dir_entry) => {
                    let entry_type = dir_entry.file_type();
                    let entry_name = dir_entry.file_name().to_str().unwrap();
                    let entry_path = dir_entry.path();
                    let dest_path = &destination;

                    let new_path = Path::new(&dest_path).join(entry_name);

                    if entry_type.is_file() {
                        report(
                            &mut activity_log,
                            format!("moving file `{}` to `{}`...\n", entry_name, dest_path)
                                .as_str(),
                            true,
                            LogLvl::Activity,
                        );

                        match move_file(entry_path, new_path, &file_copy_options) {
                            Ok(_) => report(&mut activity_log, "done\n", true, LogLvl::Activity),
                            Err(e) => {
                                report(
                                    &mut activity_log,
                                    "ERROR (see error.log)\n",
                                    true,
                                    LogLvl::Error,
                                );
                                report(
                                    &mut error_log,
                                    format!("{}\n", e).as_str(),
                                    false,
                                    LogLvl::Error,
                                );
                            }
                        }
                    } else if entry_type.is_dir() {
                        report(
                            &mut activity_log,
                            format!("moving folder `{}` to `{}`...\n", entry_name, dest_path)
                                .as_str(),
                            true,
                            LogLvl::Activity,
                        );
                        match move_dir(entry_path, dest_path, &dir_copy_options) {
                            Ok(_) => report(&mut activity_log, "done\n", true, LogLvl::Activity),
                            Err(e) => {
                                report(
                                    &mut activity_log,
                                    "ERROR (see error.log)\n",
                                    true,
                                    LogLvl::Error,
                                );
                                report(
                                    &mut error_log,
                                    format!("{}\n", e).as_str(),
                                    false,
                                    LogLvl::Error,
                                );
                            }
                        }
                        report(&mut activity_log, "done\n", true, LogLvl::Activity);
                    }
                }
                Err(e) => report(
                    &mut error_log,
                    format!("in WalkDir(): `{e}`\n").as_str(),
                    true,
                    LogLvl::Error,
                ),
            }
        }
        sleep_countdown!(sleep_time);
    }
}
