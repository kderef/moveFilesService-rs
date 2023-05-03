use serde::Deserialize;
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::Path
};

use crate::{config::{CONFIG_PATH, DEFAULT_CONFIG}, pause_exit};
use crate::LogLvl;
use crate::{file_exists, report};

#[derive(Deserialize)]
pub struct Config {
    pub seconds: u32,
    pub source: String,
    pub destination: String,
}

pub fn parse_toml_config(error_log: &mut File) -> Config {
    if !file_exists!(CONFIG_PATH) {
        report(error_log, format!("the config file `{CONFIG_PATH}` does not exist, creating it with default config...\n").as_str(), true, LogLvl::Warning);
        let conf_file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(CONFIG_PATH);

        match conf_file {
            Ok(mut fd) => match write!(&mut fd, "{}", DEFAULT_CONFIG) {
                Ok(_) => {
                    report(error_log, "the config file has been created. Please enter your values into it and restart the program.\n", true, LogLvl::Error);
                    pause_exit!();
                }
                Err(msg) => {
                    report(error_log, format!("failed to write the default config to the config file because of error: {msg}\n").as_str(), true, LogLvl::Error);
                    pause_exit!();
                }
            },
            Err(msg) => {
                report(error_log, format!("failed to open config file `{CONFIG_PATH}` for writing because of error: {msg}\n").as_str(), true, LogLvl::Error);
                pause_exit!();
            }
        }
    }
    let config_contents = std::fs::read_to_string(CONFIG_PATH).unwrap();
    if config_contents.trim().is_empty() {
        report(
            error_log,
            format!(
                "the config file `{CONFIG_PATH}` is empty, filling it with default config...\n"
            )
            .as_str(),
            true,
            LogLvl::Warning,
        );
        let conf_file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(CONFIG_PATH);

        match conf_file {
            Ok(mut fd) => match write!(&mut fd, "{}", DEFAULT_CONFIG) {
                Ok(_) => {
                    report(error_log, format!("please enter your values into the config file `{CONFIG_PATH}` and restart the program.\n").as_str(), true, LogLvl::Warning);
                    pause_exit!();
                }
                Err(msg) => {
                    report(error_log, format!("failed to open config file `{CONFIG_PATH}` for writing because of error: {msg}\n").as_str(), true, LogLvl::Warning);
                    pause_exit!();
                }
            },
            Err(msg) => {
                report(
                    error_log,
                    format!("failed to open config file `{CONFIG_PATH}` because of error: {msg}\n")
                        .as_str(),
                    true,
                    LogLvl::Error,
                );
                pause_exit!();
            }
        }
    }

    /* read with TOML */
    return match toml::from_str(&config_contents) {
        Ok(conf) => conf,
        Err(msg) => {
            report(error_log, format!("in config file `{CONFIG_PATH}`: failed to parse config because of error: {msg}\n").as_str(), true, LogLvl::Error);
            pause_exit!();
        }
    };
}
