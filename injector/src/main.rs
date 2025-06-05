use std::ffi::{c_void, CStr};
use std::mem;
use std::mem::{transmute, zeroed};
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use injector::E4RTHBYTE_SHELLCODE;
use windows::{
    Win32::System::Diagnostics::ToolHelp::{CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS},
    Win32::System::Diagnostics::Debug::{WriteProcessMemory},
    Win32::Foundation::{CloseHandle, FALSE},
    Win32::System::Threading::{OpenProcess, CreateRemoteThread, LPTHREAD_START_ROUTINE, PROCESS_ALL_ACCESS},
    Win32::System::Memory::{VirtualAllocEx, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE}
};
use windows::core::BOOL;

macro_rules! msg {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        println!("[+] {}", msg);
    }};
}

macro_rules! error {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        println!("[-] {}, exiting in 3s", msg);
        sleep(Duration::from_secs(3));
        exit(1);
    }};
}

fn get_roblox_pid() -> u32 {
    unsafe {
        let mut pid = 0;

        let Ok(h_snapshot) = CreateToolhelp32Snapshot(
            TH32CS_SNAPPROCESS,
            0)
        else {
            error!("Failed to create snapshot")
        };

        let mut pe: PROCESSENTRY32 = zeroed();
        pe.dwSize = size_of::<PROCESSENTRY32>() as u32;

        let mut success = Process32First(h_snapshot, &mut pe);

        while success.is_ok() {
            if CStr::from_ptr(pe.szExeFile.as_ptr()).to_string_lossy().eq_ignore_ascii_case("RobloxStudioBeta.exe") {
                pid = pe.th32ProcessID;
                break;
            }

            success = Process32Next(h_snapshot, &mut pe);
        }

        let _ = CloseHandle(h_snapshot);

        pid
    }
}

fn main() {
    unsafe {
        let roblox_pid = get_roblox_pid();

        if roblox_pid == 0 {
            error!("Roblox not found!");
        }

        msg!("PID: {}", roblox_pid);

        let Ok(h_process) = OpenProcess(
            PROCESS_ALL_ACCESS,
            FALSE.as_bool(),
            roblox_pid)
        else {
            error!("Failed to open process");
        };

        let buffer = E4RTHBYTE_SHELLCODE;
        let buffer_size = buffer.len();

        msg!("Buffer size: {}", buffer_size);

        let address = VirtualAllocEx(
            h_process, None, buffer_size,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_EXECUTE_READWRITE);

        if address.is_null() {
            let _ = CloseHandle(h_process);
            error!("Failed to allocate memory");
        }

        msg!("Memory allocated, address: {:?}", address);

        if WriteProcessMemory(
            h_process, address, buffer.as_ptr() as _,
            buffer_size, Some(&mut 0)).is_err() 
        {
            let _ = CloseHandle(h_process);
            error!("Failed to write to process");
        }
        
        msg!("Memory wrote!");

        if CreateRemoteThread(
            h_process, None, 0,
            transmute::<*mut c_void, LPTHREAD_START_ROUTINE>(address),
            None, 0, None
        ).is_err() {
            let _ = CloseHandle(h_process);
            error!("Failed to create remote thread");
        }
        
        msg!("Injected!");
        
        let _ = CloseHandle(h_process);
    }
}
