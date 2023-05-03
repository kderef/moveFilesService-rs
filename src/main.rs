#![allow(non_snake_case)]

extern crate fs_extra;
extern crate serde;
extern crate walkdir;

mod config;
mod movefiles;
mod parse_config;
mod time_utils;
mod util;

use chrono::Datelike;
use colored::Colorize;
use config::*;
use fs_extra::{dir::move_dir, file::move_file};
use movefiles::*;
use parse_config::parse_toml_config;
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

    let file_copy_options = fs_extra::file::CopyOptions::new().overwrite(true);
    let dir_copy_options = fs_extra::dir::CopyOptions::new().overwrite(true);

    let toml_config = parse_toml_config(&mut error_log);

    let mut source = toml_config.source;
    let mut destination = toml_config.destination;
    let sleep_time = toml_config.seconds;

    if sleep_time > SECONDS_MAX {
        report(
            &mut error_log,
            format!(
                "in file `{CONFIG_PATH}`: key \"seconds\" should be > 0s and < {}s ({}h)\n",
                SECONDS_MAX,
                SECONDS_MAX / 3600
            )
            .as_str(),
            true,
            LogLvl::Error,
        );
        pause_exit!("press any key to exit . . .");
    }

    if !source.ends_with(MAIN_SEPARATOR) && !source.trim().is_empty() {
        source.push(MAIN_SEPARATOR);
    }
    if !destination.ends_with(MAIN_SEPARATOR) && !destination.trim().is_empty() {
        destination.push(MAIN_SEPARATOR);
    }

    if !dir_exists!(&source) {
        report(
            &mut error_log,
            format!(
                "in config file `{CONFIG_PATH}`: the source directory '{source}' does not exist."
            )
            .as_str(),
            true,
            LogLvl::Error,
        );
        pause_exit!("press any key to exit . . .");
    }
    if !dir_exists!(&destination) {
        report(&mut error_log, format!("in config file `{CONFIG_PATH}`: the destination directory '{destination}' does not exist.").as_str(), true, LogLvl::Error);
        pause_exit!("press any key to exit . . .");
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

                    let new_path = Path::new(&destination).join(entry_name);

                    if entry_type.is_file() {
                        report(
                            &mut activity_log,
                            format!("moving file `{}` to `{}`...\n", entry_name, &destination)
                                .as_str(),
                            true,
                            LogLvl::Activity,
                        );

                        match move_file(entry_path, new_path, &file_copy_options) {
                            Ok(_) => report(&mut activity_log, "done\n\n", true, LogLvl::Activity),
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
                            format!("moving folder `{}` to `{}`...\n", entry_name, &destination)
                                .as_str(),
                            true,
                            LogLvl::Activity,
                        );
                        match move_dir(entry_path, &destination, &dir_copy_options) {
                            Ok(_) => report(&mut activity_log, "done\n\n", true, LogLvl::Activity),
                            Err(e) => {
                                report(
                                    &mut activity_log,
                                    "ERROR (see error.log)\n\n",
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
                        report(&mut activity_log, "done\n\n", true, LogLvl::Activity);
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
