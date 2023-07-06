#![allow(non_snake_case)]

extern crate fs_extra;
extern crate serde;
extern crate walkdir;

mod config;
mod logger;
mod parse_config;
mod util;

use crate::parse_config::config_path;
use colored::Colorize;
use fs_extra::{dir::move_dir, file::move_file};
use logger::Logger;
use walkdir::WalkDir;

use config::*;
use parse_config::parse_toml_config;
use is_terminal::IsTerminal;
use std::path::MAIN_SEPARATOR;
use std::{io::Write, path::Path};

const DATE_FMT: &str = "%d-%m-%Y";

fn main() {
    let is_term = std::io::stdout().is_terminal();
    let conf_path = config_path();

    /* initialize logging */
    let mut log = Logger::new(DATE_FMT, LOG_EXT, ACTIVITY_LOG_PATH, ERROR_LOG_PATH);

    if is_term {
        println!(
            "{LOGO} [{}{}]\n{SEPARATOR_LONG}",
            "v".yellow(),
            VERSION.yellow()
        );
        println!(
            "{}: {}\n",
            "start date".yellow(),
            chrono_time!(TIME_FORMAT).green()
        );
    }


    // check for config files (source.txt, destination) //

    let file_copy_options = fs_extra::file::CopyOptions::new().overwrite(true);
    let dir_copy_options = fs_extra::dir::CopyOptions::new().overwrite(true);

    let toml_config = parse_toml_config(&mut log);

    let (mut source, mut destination, sleep_time) = toml_config.into();

    if sleep_time > SECONDS_MAX {
        log.err(&format!(
            "in file `{}`: key \"seconds\" should be > 0s and < {SECONDS_MAX}s ({SECONDS_MAX_H}h)\n", conf_path.display()
        ));

        pause_exit!();
    }

    if !source.ends_with(MAIN_SEPARATOR) && !source.trim().is_empty() {
        source.push(MAIN_SEPARATOR);
    }
    if !destination.ends_with(MAIN_SEPARATOR) && !destination.trim().is_empty() {
        destination.push(MAIN_SEPARATOR);
    }

    if !dir_exists!(&source) {
        log.err(&format!(
            "in config file `{}`: the source directory '{source}' does not exist.", conf_path.display()
        ));

        pause_exit!();
    }
    if !dir_exists!(&destination) {
        log.err(&format!("in config file `{}`: the destination directory '{destination}' does not exist.", conf_path.display()));
        pause_exit!();
    }

    println!("{}:\t\t{}", "source dir".yellow(), source);
    println!("{}:\t{}", "destination dir".yellow(), destination);
    println!("{SEPARATOR}");

    loop {
        for item in WalkDir::new(&source).min_depth(1) {
            match item {
                Ok(dir_entry) => {
                    let entry_type = dir_entry.file_type();
                    let entry_name = dir_entry.file_name().to_str().unwrap();
                    let entry_path = dir_entry.path();

                    let new_path = Path::new(&destination).join(entry_name);

                    if entry_type.is_file() {
                        log.info(&format!(
                            "moving file `{entry_name}` to `{}`...",
                            &destination
                        ));

                        match move_file(entry_path, new_path, &file_copy_options) {
                            Ok(_) => log.info("done\n\n"),
                            Err(e) => {
                                log.info("ERROR (see error.log)\n\n");
                                log.err(&format!("{e}\n"));
                            }
                        }
                    } else if entry_type.is_dir() {
                        log.info(&format!(
                            "moving folder `{entry_name}` to `{}`...\n",
                            &destination
                        ));

                        match move_dir(entry_path, &destination, &dir_copy_options) {
                            Ok(_) => log.info("done.\n\n"),
                            Err(e) => {
                                log.info("ERROR (see error.log)\n\n");
                                log.err(&format!("{e}\n"));
                            }
                        }
                        log.info("done.\n\n");
                    }
                }
                Err(e) => log.err(&format!("in WalkDir(): `{e}`\n"))
            }
        }
        sleep_countdown!(sleep_time);
    }
}
