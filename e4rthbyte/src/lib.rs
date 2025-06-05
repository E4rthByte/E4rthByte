use std::ptr::null_mut;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    MessageBoxA, MB_OK,
};

#[unsafe(no_mangle)]
pub extern "C" fn entrypoint() {
    unsafe {
        MessageBoxA(
            null_mut(),
            b"E4rthByte!\0".as_ptr() as _,
            b"Injected!!\0".as_ptr() as _,
            MB_OK,
        );
    }
}