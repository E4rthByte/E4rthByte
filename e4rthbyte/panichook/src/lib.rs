#![feature(backtrace_frames)]

use std::backtrace::Backtrace;
use std::ffi::CString;
use std::panic::PanicHookInfo;
use windows::core::PCSTR;
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_ICONERROR, MB_OK};

pub fn messagebox_panic_hook(info: &PanicHookInfo<'_>) {
    let location = match info.location() {
        None => "<unknown location>".to_string(),
        Some(location) => format!("{}:{}", location.file(), location.line()),
    };
    
    let message = if let Some(s) = info.payload().downcast_ref::<&str>() {
        *s
    } else if let Some(s) = info.payload().downcast_ref::<String>() {
        s
    } else {
        "<unknown>"
    };
    
    let message = format!(
        "{}\n\n\
        Location: \n{}\n\n\
        Backtrace: \n{}",
        message,
        location,
        Backtrace::force_capture()
    );
    
    let message = CString::new(message).unwrap();
    let title = b"E4rthByte: Panic\0";
    
    unsafe {
        MessageBoxA(
            None,
            PCSTR(message.as_ptr() as _),
            PCSTR(title.as_ptr() as _),
            MB_OK | MB_ICONERROR
        );
    }
}