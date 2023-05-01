use crate::config::*;
use crate::timestamp;
use std::path::Path;

/// test if `path` is a file and exists
#[macro_export]
macro_rules! file_exists {
    ($path:expr) => {
        Path::new($path).is_file()
    };
}

/// test if `path` exists and is a dir
#[macro_export]
macro_rules! dir_exists {
    ($path:expr) => {
        Path::new($path).is_dir()
    };
}

/// returns file in format `{path_} - [%d-%m-%Y]`
pub fn timestamped_path(path_: &str) -> String {
    let p_ = Path::new(path_);
    let stem = p_.file_stem().unwrap().to_str().unwrap();
    let ext = p_.extension();

    if ext.is_none() {
        format!("{} - [{}]", stem, timestamp!(PATH_TIME_FORMAT))
    } else {
        format!(
            "{} - [{}].{}",
            stem,
            timestamp!(PATH_TIME_FORMAT),
            ext.unwrap().to_str().unwrap()
        )
    }
}