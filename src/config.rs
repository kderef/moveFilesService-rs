/// the file name/path to the error log, errors will be logged to this path.
pub const ERROR_LOG_PATH: &str = "error.log";

/// the file name/path to the activity log, this log contains all file operations (move_file, move_dir, etc.)
pub const ACTIVITY_LOG_PATH: &str = "activity.log";

/// the file name/path to the file containing the source directory.
pub const SOURCE_FILE_PATH: &str = "source.txt";

/// the file name/path to the file containing the destination directory.
pub const DESTINATION_FILE_PATH: &str = "destination.txt";

/// the file name/path to the file containing the amount of seconds the program should sleep after completing an iteration.
pub const SECONDS_FILE_PATH: &str = "seconds.txt";

/// the format of the timestamp used in `report()`
pub const TIME_FORMAT: &str = "[%d-%m-%Y ~ %H:%M:%S]";

/// format used in filenames, e.g. `errors - [PATH_TIME_FORMAT].log`
pub const PATH_TIME_FORMAT: &str = "%d-%m-%Y";

/// the max number of seconds a user can set in the `seconds.txt` file.
pub const SECONDS_MAX: u64 = 60 * 60 * 24;

/// the logo that is printen upon first startup
pub const LOGO: &str = "\
___  ___                 ______ _ _           
|  \\/  |                |  ____(_) |          
| \\  / | _____   _____  | |__   _| | ___  ___ 
| |\\/| |/ _ \\ \\ / / _ \\ |  __| | | |/ _ \\/ __|
| |  | | (_) \\ V /  __/ | |    | | |  __/\\__ \\
|_|  |_|\\___/ \\_/ \\___| |_|    |_|_|\\___||___/
\n  [Kian Heitkamp] [rust]\n";

/// separator
pub const SEPARATOR: &str = "===================================";