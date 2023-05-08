/// the version specified in Cargo.toml (statically read)
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// the file name/path to the error log, errors will be logged to this path.
pub const ERROR_LOG_PATH: &str = "error.log";

/// separator for add_sep function
pub const LOG_SEPARATOR: &str = "\n\n";

/// the file name/path to the activity log, this log contains all file operations (move_file, move_dir, etc.)
pub const ACTIVITY_LOG_PATH: &str = "activity.log";

/// path to the config file
pub const CONFIG_PATH: &str = "config.ini";

/// default TOML config file
#[cfg(target_os = "windows")]
pub const DEFAULT_CONFIG: &str = r"seconds = 60

# example:
# source = 'C:\temp\'
# destination = 'U:\target\'
source = ''
destination = ''";

#[cfg(target_os = "linux")]
pub const DEFAULT_CONFIG: &str = r"seconds = 60

# example:
# source = '/home/<username>/Documents/map-1'
# destination = '/home/<username>/Documents/map-2'
source = ''
destination = ''";

#[cfg(target_os = "macos")]
pub const DEFAULT_CONFIG: &str = r"seconds = 60

# example:
# source = '/Users/<username>/Documents/map-1'
# destination = '/Users/<username>/Documents/map-2'
source = ''
destination = ''";


/// the format of the timestamp used in `report()`
pub const TIME_FORMAT: &str = "[%d-%m-%Y ~ %H:%M:%S]";

/// format used in filenames, e.g. `errors - [PATH_TIME_FORMAT].log`
pub const PATH_TIME_FORMAT: &str = "%d-%m-%Y";

/// the max number of seconds a user can set in the config file
pub const SECONDS_MAX: u32 = 60 * 60 * 24;

/// the logo that is printed upon first startup
pub const LOGO: &str = r"
$$\      $$\                               $$$$$$$$\ $$\ $$\                     
$$$\    $$$ |                              $$  _____|\__|$$ |                    
$$$$\  $$$$ | $$$$$$\ $$\    $$\  $$$$$$\  $$ |      $$\ $$ | $$$$$$\   $$$$$$$\ 
$$\$$\$$ $$ |$$  __$$\\$$\  $$  |$$  __$$\ $$$$$\    $$ |$$ |$$  __$$\ $$  _____|
$$ \$$$  $$ |$$ /  $$ |\$$\$$  / $$$$$$$$ |$$  __|   $$ |$$ |$$$$$$$$ |\$$$$$$\  
$$ |\$  /$$ |$$ |  $$ | \$$$  /  $$   ____|$$ |      $$ |$$ |$$   ____| \____$$\ 
$$ | \_/ $$ |\$$$$$$  |  \$  /   \$$$$$$$\ $$ |      $$ |$$ |\$$$$$$$\ $$$$$$$  |
\__|     \__| \______/    \_/     \_______|\__|      \__|\__| \_______|\_______/  ";

/// separator
pub const SEPARATOR: &str = "===================================";

/// separator for logo
pub const SEPARATOR_LONG: &str =
    "===========================================================================================";
