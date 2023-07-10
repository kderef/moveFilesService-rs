use serde::Deserialize;
use std::{
    fs::OpenOptions,
    io::Write, path::{Path, PathBuf}
};

use crate::{
    config::{DEFAULT_CONFIG, CONFIG_LOCATION},
    pause_exit,
};
use crate::file_exists;
use crate::logging::*;

#[derive(Deserialize)]
pub struct Config {
    pub seconds: u32,
    pub source: String,
    pub destination: String,
}

impl Into<(String, String, u32)> for Config {
    fn into(self) -> (String, String, u32) {
        (self.source, self.destination, self.seconds)
    }
}

pub fn config_path() -> PathBuf {
    Path::new(CONFIG_LOCATION).to_path_buf()
}

pub fn parse_toml_config() -> Config {
    let conf_path = config_path().join("config.ini");

    if !file_exists!(conf_path.as_path()) {
        wrn(format!("the config file `{}` does not exist, creating it with default config...\n", conf_path.display()));

        let conf_file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&conf_path);

        match conf_file {
            Ok(mut fd) => match write!(&mut fd, "{}", DEFAULT_CONFIG) {
                Ok(_) => {
                    wrn("the config file has been created. Please enter your values into it and restart the program.\n".into());
                    pause_exit!();
                }
                Err(msg) => {
                    err(format!("failed to write the default config to the config file because of error: {msg}\n"));
                    pause_exit!();
                }
            },
            Err(msg) => {
                err(format!("failed to open config file `{}` for writing because of error: {msg}\n", conf_path.display()));
                pause_exit!();
            }
        }
    }
    let config_contents = std::fs::read_to_string(&conf_path).unwrap();
    if config_contents.trim().is_empty() {
        wrn(
            format!(
                "the config file `{}` is empty, filling it with default config...\n", conf_path.display()
            ),
        );
        let conf_file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&conf_path);

        match conf_file {
            Ok(mut fd) => match write!(&mut fd, "{}", DEFAULT_CONFIG) {
                Ok(_) => {
                    wrn(format!("please enter your values into the config file `{}` and restart the program.\n", conf_path.display()));
                    pause_exit!();
                }
                Err(msg) => {
                    err(format!("failed to open config file `{}` for writing because of error: {msg}\n", conf_path.display()));
                    pause_exit!();
                }
            },
            Err(msg) => {
                err(format!("failed to open config file `{}` because of error: {msg}\n", conf_path.display()));
                pause_exit!();
            }
        }
    }

    /* read with TOML */
    return match toml::from_str(&config_contents) {
        Ok(conf) => conf,
        Err(msg) => {
            err(format!("in config file `{}`: failed to parse config because of error: {msg}\n", conf_path.display()));
            pause_exit!();
        }
    };
}
