use injector::{error, get_pid, msg, E4RTHBYTE_SHELLCODE};
use std::ffi::{c_void, CStr};
use std::mem::{transmute, zeroed};
use windows::Win32::Foundation::{CloseHandle, FALSE};
use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;
use windows::Win32::System::Threading::{CreateRemoteThread, OpenProcess, LPTHREAD_START_ROUTINE, PROCESS_ALL_ACCESS};
use windows::Win32::System::Memory::{VirtualAllocEx, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE};

fn main() {
    unsafe {
        let Some(roblox_pid) = get_pid("RobloxStudioBeta.exe") else {
            error!("Roblox not found!");
        };

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
            PAGE_EXECUTE_READWRITE
        );

        if address.is_null() {
            let _ = CloseHandle(h_process);
            error!("Failed to allocate memory");
        }

        msg!("Memory allocated, address: {:?}", address);

        if WriteProcessMemory(
            h_process, address, buffer.as_ptr() as _,
            buffer_size, Some(&mut 0)
        ).is_err() {
            let _ = CloseHandle(h_process);
            error!("Failed to write to process");
        }
        
        msg!("Memory wrote!");

        if CreateRemoteThread(
            h_process, None, 
            0,
            transmute::<*mut c_void, LPTHREAD_START_ROUTINE>(address),
            None, 
            0, 
            None
        ).is_err() {
            let _ = CloseHandle(h_process);
            error!("Failed to create remote thread");
        }
        
        msg!("Injected!");
        
        let _ = CloseHandle(h_process);
    }
}
