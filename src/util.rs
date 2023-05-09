/// sleep for `$sleep_time` seconds, continuously printing how much time is left.
#[macro_export]
macro_rules! sleep_countdown {
    ($sleep_time:expr) => {
        for i in (1..=($sleep_time)).rev() {
            print!(
                "completed, sleeping for {}                          \r",
                format!("{}s", i).green()
            );
            std::io::stdout().flush().unwrap();
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    };
}

/// if parameter $msg is given, print that message and run CMD.exe /c pause
///
/// if no parameters are given, pause with the default message.
#[cfg(target_os = "windows")]
#[macro_export]
macro_rules! pause_exit {
    () => {
        print!("press any key to exit . . .");
        std::io::stdout().flush().unwrap();
        let _ = std::process::Command::new("cmd.exe").args(["/c", "pause > nul"]).status();
        std::process::exit(1);
    };
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
#[macro_export]
macro_rules! pause_exit {
    () => {
            let _ = std::process::Command::new("/usr/bin/read").args(["-n 1", "-p press any key to exit . . ."]).status();
            std::process::exit(1);
    };
}

/// test if `path` is a file and exists
#[macro_export]
macro_rules! file_exists {
    ($path:expr) => {
        std::path::Path::new($path).is_file()
    };
}

/// test if `path` exists and is a dir
#[macro_export]
macro_rules! dir_exists {
    ($path:expr) => {
        std::path::Path::new($path).is_dir()
    };
}