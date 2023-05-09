# moveFilesService-rs
a service that continuesly moves files and folders from one directory to another.

## Usage:
1. download the program from the [releases tab](https://github.com/Kn-Ht/moveFilesService-rs/releases) or [build it from source](https://github.com/Kn-Ht/moveFilesService-rs#compilation)
2. run the exe, it will generate the necessary files and generate an empty config.
3. insert your values into the config file.
4. restart the program.

---
## Compilation
**prerequisites: [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) and [git CLI](https://cli.github.com/)**
```bash
git clone https://github.com/Kn-Ht/moveFilesService-rs
cd moveFilesService-rs
cargo build --release
# the executable will be in the ./target/release/ directory.
```
---
## Supported Platforms
- Windows
- Macos
- Linux
