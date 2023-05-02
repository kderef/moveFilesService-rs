/// get current day 1 - 31
#[macro_export]
macro_rules! current_day {
    () => {
        chrono::Local::now().day()
    };
}

/// timestamp used for logging purposes
#[macro_export]
macro_rules! timestamp {
    () => {
        chrono::Local::now().format(TIME_FORMAT)
    };
    ($time_fmt:expr) => {
        chrono::Local::now().format($time_fmt)
    };
}
