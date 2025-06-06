use std::ffi::CString;
use std::panic::PanicHookInfo;
use windows::core::PCSTR;
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_ICONERROR, MB_OK};

pub fn messagebox_panic_hook(info: &PanicHookInfo<'_>) {
    let location = info.location()
        .map(|l| format!("{}:{}", l.file(), l.line()))
        .unwrap_or_else(|| "<unknown location>".to_string());

    let message = if let Some(s) = info.payload().downcast_ref::<&str>() {
        *s
    } else if let Some(s) = info.payload().downcast_ref::<String>() {
        s
    } else {
        "<unknown>"
    };
    
    let message = format!(
        "{}\n\n\
        Location: \n{}",
        message,
        location
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