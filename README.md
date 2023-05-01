# moveFilesService-rs
a service that continuesly moves files and folders from one directory to another.

## USAGE:
- create config files in the same directory, called `seconds.txt`, `source.txt` and `destination.txt`

`seconds.txt`: the number of seconds to sleep after each loop iteration, must be >0s and < config::SECONDS_MAX

`source.txt`: the source directory where all items will be moved from

`destination.txt`: the destination directory where all items will be moved to

---
## COMPILATION
**prerequisites: [cargo](https://doc.rust-lang.org/cargo/) and [git CLI](https://cli.github.com/)**
```bash
git clone https://github.com/x-kvoid-x/moveFilesService-rs
cd moveFilesService-rs
cargo build --release
# the executable will be in the ./target/release/ directory.
```


---
## INSTALLATION
install the .exe from the `releases` tab, create the necessary files and then run it.
