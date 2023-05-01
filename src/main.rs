#![allow(non_snake_case)]

extern crate fs_extra;
extern crate walkdir;

mod config;
mod movefiles;
mod util;

use chrono::Datelike;
use colored::Colorize;
use config::*;
use fs_extra::{dir::move_dir, file::move_file};
use movefiles::*;
use std::process::exit;
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
};
use util::{report, LogLvl};

fn main() {
    println!("{LOGO} [{}{}]\n{SEPARATOR_LONG}", "v".yellow(), VERSION.yellow());
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
    activity_log = open_activity_log!(activity_log_path.clone(), &mut error_log);

    // check for config files (source.txt, destination) //

    let source; // source directory
    let destination; // destination directory
    let sleep_time; // amount of time to sleep after loop{} iteration

    let file_copy_options = fs_extra::file::CopyOptions::new().overwrite(true);
    let dir_copy_options = fs_extra::dir::CopyOptions::new().overwrite(true);

    // check if the files exist //
    match try_get_seconds() {
        Ok(seconds) => sleep_time = seconds,
        Err(msg) => {
            report(&mut error_log, msg.as_str(), true, LogLvl::Error);
            pause!("press any key to exit . . .");
            exit(0);
        }
    }
    match try_get_source() {
        Ok(path) => source = path,
        Err(msg) => {
            report(&mut error_log, msg.as_str(), true, LogLvl::Error);
            pause!("press any key to exit . . .");
            exit(0);
        }
    }
    match try_get_destination() {
        Ok(path) => destination = path,
        Err(msg) => {
            report(&mut error_log, msg.as_str(), true, LogLvl::Error);
            pause!("press any key to exit . . .");
            exit(0);
        }
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
            error_log = open_error_log!(&error_log_path.clone());
        }
        if !file_exists!(&activity_log_path.clone()) {
            activity_log = open_activity_log!(&activity_log_path.clone(), error_log);
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
