#![allow(non_snake_case)]

extern crate fs_extra;
extern crate serde;
extern crate walkdir;

mod config;
mod logger;
mod movefiles;
mod parse_config;
mod time_utils;
mod util;

use colored::Colorize;
use fs_extra::{dir::move_dir, file::move_file};
use logger::Logger;

use config::*;
use parse_config::parse_toml_config;
use std::path::MAIN_SEPARATOR;
use std::{
    io::Write,
    path::Path,
};

fn main() {
    /* initialize logging */
    let mut log = Logger::new(
        "%d-%m-%Y",
        "log",
        "activity",
        "error",
    );

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

    // check for config files (source.txt, destination) //

    let file_copy_options = fs_extra::file::CopyOptions::new().overwrite(true);
    let dir_copy_options = fs_extra::dir::CopyOptions::new().overwrite(true);

    let toml_config = parse_toml_config(&mut log);

    let mut source = toml_config.source;
    let mut destination = toml_config.destination;
    let sleep_time = toml_config.seconds;

    if sleep_time > SECONDS_MAX {
        log.err(
            format!(
                "in file `{CONFIG_PATH}`: key \"seconds\" should be > 0s and < {}s ({}h)\n",
                SECONDS_MAX,
                SECONDS_MAX / 3600
            )
            .as_str(),
        );

        pause_exit!();
    }

    if !source.ends_with(MAIN_SEPARATOR) && !source.trim().is_empty() {
        source.push(MAIN_SEPARATOR);
    }
    if !destination.ends_with(MAIN_SEPARATOR) && !destination.trim().is_empty() {
        destination.push(MAIN_SEPARATOR);
    }

    if !dir_exists!(&source) {
        log.err(
            format!(
                "in config file `{CONFIG_PATH}`: the source directory '{source}' does not exist."
            )
            .as_str()
        );

        pause_exit!();
    }
    if !dir_exists!(&destination) {
        log.err(format!("in config file `{CONFIG_PATH}`: the destination directory '{destination}' does not exist.").as_str());
        pause_exit!();
    }

    println!("{}:\t\t{}", "source dir".yellow(), source);
    println!("{}:\t{}", "destination dir".yellow(), destination);
    println!("{SEPARATOR}");

    loop {
        for item in walkdir::WalkDir::new(&source).min_depth(1) {
            match item {
                Ok(dir_entry) => {
                    let entry_type = dir_entry.file_type();
                    let entry_name = dir_entry.file_name().to_str().unwrap();
                    let entry_path = dir_entry.path();

                    let new_path = Path::new(&destination).join(entry_name);

                    if entry_type.is_file() {
                        log.info(format!("moving file `{}` to `{}`...", entry_name, &destination).as_str());

                        match move_file(entry_path, new_path, &file_copy_options) {
                            Ok(_) => log.info("done\n\n"),
                            Err(e) => {
                                log.info("ERROR (see error.log)\n\n");
                                log.err(format!("{e}\n").as_str());
                            }
                        }
                    } else if entry_type.is_dir() {
                        log.info(format!("moving folder `{}` to `{}`...\n", entry_name, &destination).as_str());

                        match move_dir(entry_path, &destination, &dir_copy_options) {
                            Ok(_) => log.info("done.\n\n"),
                            Err(e) => {
                                log.info("ERROR (see error.log)\n\n");
                                log.err(format!("{e}\n").as_str());
                            }
                        }
                        log.info("done.\n\n");
                    }
                }
                Err(e) => log.err(format!("in WalkDir(): `{e}`\n").as_str())
            }
        }
        sleep_countdown!(sleep_time);
    }
}
