extern crate libc;
use libc::{process_vm_readv, process_vm_writev};
use std::process::{Command};

const MAX_PID: i32 = 32768;

pub fn inject_and_migrate(shellcode: &[u8]) {
    println!("Printing a {}", "thing");
    let mut pid = 2;
    // End up changing this to less than MAX_PID
    while pid < 2000 {
        let kill = Command::new("kill")
            .arg("-s")
            .arg("0")
            .arg(pid.to_string())
            .output()
            .unwrap();
        match kill.stderr.len() {
            0 => {
                // look for ptrace_seize or (preferably!) process_vm_readv and break if found
                //println!("candidate: {}", &pid);
                pid += 1
            },
            _ => pid += 1
        }
    }
    // Try http://phrack.org/issues/59/12.html but with https://man7.org/linux/man-pages/man2/process_vm_readv.2.html
    //let test = unsafe { libc::process_vm_readv() };
}
