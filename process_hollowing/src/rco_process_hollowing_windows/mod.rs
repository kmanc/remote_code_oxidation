extern crate windows;
use windows::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory};
use windows::Win32::System::Threading::{CreateProcessW, NtQueryInformationProcess, PROCESS_BASIC_INFORMATION, PROCESS_INFORMATION, ResumeThread, STARTUPINFOW};

// CreateProcessW
// WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw
// RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/fn.CreateProcessW.html

// StartupInfoW
// WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/ns-processthreadsapi-startupinfow
// RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/struct.STARTUPINFOW.html

// ProcessInformation
// WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/ns-processthreadsapi-process_information
// RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/struct.PROCESS_INFORMATION.html

// zwQueryInformation
// WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/procthread/zwqueryinformationprocess
// RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/fn.NtQueryInformationProcess.html

// Process_Basic_Information
// WINDOWS --> https://www.pinvoke.net/default.aspx/Structures/PROCESS_BASIC_INFORMATION.html
// RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/struct.PROCESS_BASIC_INFORMATION.html

// ReadProcessMemory
// WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-readprocessmemory
// RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Diagnostics/Debug/fn.ReadProcessMemory.html

// WriteProcessMemory
// WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-writeprocessmemory
// RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Diagnostics/Debug/fn.WriteProcessMemory.html

// ResumeThread
// WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-resumethread
// RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/fn.ResumeThread.html


pub fn hollow_and_run(shellcode: &[u8]) {
    println!("stub");
}