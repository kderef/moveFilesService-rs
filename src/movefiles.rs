use crate::config::{
    DESTINATION_FILE_PATH, PATH_TIME_FORMAT, SECONDS_FILE_PATH, SECONDS_MAX, SOURCE_FILE_PATH,
};
use crate::timestamp;
use std::fs;
use std::path::{Path, MAIN_SEPARATOR};

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
/// try to get the source dir from the path config::SOURCE_FILE_PATH
pub fn try_get_source() -> Result<String, String> {
    if !file_exists!(SOURCE_FILE_PATH) {
        return Err(format!(
            "in try_get_source(): the source file `{}` does not exist.\n",
            SOURCE_FILE_PATH
        ));
    }
    let file_contents = std::fs::read_to_string(SOURCE_FILE_PATH).unwrap();
    let mut file_contents_str = file_contents.as_str().trim().to_owned();

    if !file_contents_str.ends_with(MAIN_SEPARATOR) {
        file_contents_str.push(MAIN_SEPARATOR);
    }

    if file_contents.trim().is_empty() {
        return Err(format!(
            "in try_get_source(): the source file `{}` is empty.\n",
            SOURCE_FILE_PATH
        ));
    }
    if !dir_exists!(file_contents.as_str()) {
        return Err(format!(
            "in try_get_source(): the dir `{file_contents}` in the source file `{SOURCE_FILE_PATH}` does not exist or is invalid.\n"));
    }

    Ok(file_contents)
}

pub fn try_get_destination() -> Result<String, String> {
    if !file_exists!(DESTINATION_FILE_PATH) {
        return Err(format!("in try_get_destination(): the destination file `{DESTINATION_FILE_PATH}` does not exist.\n"));
    }
    let file_contents = fs::read_to_string(DESTINATION_FILE_PATH).unwrap();
    if file_contents.trim().is_empty() {
        return Err(format!(
            "in try_get_destination(): the destination file `{DESTINATION_FILE_PATH}` is empty.\n"
        ));
    }
    let mut file_contents_str = file_contents.as_str().trim().to_owned();

    if !file_contents_str.ends_with(MAIN_SEPARATOR) {
        file_contents_str.push(MAIN_SEPARATOR);
    }

    if !dir_exists!(file_contents.as_str()) {
        match fs::create_dir(&file_contents) {
            Ok(_) => {}
            Err(msg) => {
                return Err(format!(
                    "in try_get_destination(): failed to create_dir({}), error: {}\n",
                    &file_contents, msg
                ))
            }
        }
        return Err(format!("in try_get_destination(): the dir `{}` in the destination file `{}` does not exist or is invalid, it will be created.\n", file_contents, DESTINATION_FILE_PATH));
    }

    Ok(file_contents)
}

/// try to get the amount of sleep time
pub fn try_get_seconds() -> Result<u64, String> {
    if !file_exists!(SECONDS_FILE_PATH) {
        return Err(format!(
            "in try_get_seconds(): file `{}` does not exist.\n",
            SECONDS_FILE_PATH
        ));
    }
    let file_contents = fs::read_to_string(SECONDS_FILE_PATH).unwrap();

    let contents_parsed = file_contents.trim().parse::<i64>();
    if contents_parsed.is_err() {
        return Err(format!(
            "in try_get_seconds(): in file `{}`: `{}` is not a valid number.\n",
            SECONDS_FILE_PATH, file_contents
        ));
    }
    let seconds: i64 = contents_parsed.unwrap();
    if seconds >= 0 && seconds <= SECONDS_MAX as i64 {
        Ok(seconds as u64)
    } else {
        Err(format!(
            "in try_get_seconds(): in file `{}`: `{}` is not a valid number. (must be >= 0s and <= {}s({}h))\n",
            SECONDS_FILE_PATH,
            file_contents.trim(),
            SECONDS_MAX, SECONDS_MAX / 3600
        ))
    }
}
