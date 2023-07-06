#![allow(dead_code)]

/// the version specified in Cargo.toml (statically read)
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// log extension
pub const LOG_EXT: &str = "log";

/// the file name/path to the error log, errors will be logged to this path.
pub const ERROR_LOG_PATH: &str = "Error";

/// separator for add_sep function
pub const LOG_SEPARATOR: &str = "\n\n";

/// the file name/path to the activity log, this log contains all file operations (move_file, move_dir, etc.)
pub const ACTIVITY_LOG_PATH: &str = "Activity";

/// default TOML config file
pub const DEFAULT_CONFIG: &str = if cfg!(target_os = "windows") {
    include_str!("default-configs/default-conf-windows.ini")
} else if cfg!(target_os = "linux") {
    include_str!("default-configs/default-conf-linux.ini")
} else if cfg!(target_os = "macos") {
    include_str!("default-configs/default-conf-macos.ini")
} else {
    ""
};

/// the format of the timestamp used in `report()`
pub const TIME_FORMAT: &str = "[%d-%m-%Y ~ %H:%M:%S]";

/// format used in filenames, e.g. `errors - [PATH_TIME_FORMAT].log`
pub const PATH_TIME_FORMAT: &str = "%d-%m-%Y";

/// the max number of seconds a user can set in the config file
pub const SECONDS_MAX: u32 = 60 * 60 * 24;
pub const SECONDS_MAX_H: u16 = SECONDS_MAX as u16 / 3600;

/// the logo that is printed upon first startup
pub const LOGO: &str = r"
$$\      $$\                               $$$$$$$$\ $$\ $$\                     
$$$\    $$$ |                              $$  _____|\__|$$ |                    
$$$$\  $$$$ | $$$$$$\ $$\    $$\  $$$$$$\  $$ |      $$\ $$ | $$$$$$\   $$$$$$$\ 
$$\$$\$$ $$ |$$  __$$\\$$\  $$  |$$  __$$\ $$$$$\    $$ |$$ |$$  __$$\ $$  _____|
$$ \$$$  $$ |$$ /  $$ |\$$\$$  / $$$$$$$$ |$$  __|   $$ |$$ |$$$$$$$$ |\$$$$$$\  
$$ |\$  /$$ |$$ |  $$ | \$$$  /  $$   ____|$$ |      $$ |$$ |$$   ____| \____$$\ 
$$ | \_/ $$ |\$$$$$$  |  \$  /   \$$$$$$$\ $$ |      $$ |$$ |\$$$$$$$\ $$$$$$$  |
\__|     \__| \______/    \_/     \_______|\__|      \__|\__| \_______|\_______/   ";

/// separator
pub const SEPARATOR: &str = "===================================";

/// separator for logo
pub const SEPARATOR_LONG: &str =
    "============================================================================================";
