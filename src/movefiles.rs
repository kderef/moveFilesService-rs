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