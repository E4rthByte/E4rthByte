use std::ffi::{c_char, CString};
use std::{mem, ptr};
use windows::{
    core::*,
    Win32::System::LibraryLoader::GetModuleHandleA,
    Win32::System::Threading::{ExitProcess},
};

mod offsets;

type PRINT_FN = unsafe extern "cdecl" fn(r#type: u32, message: *const c_char, ...) -> i64;

pub fn print(format: u32, msg: &str) {
   unsafe {
       let Ok(h_base) = GetModuleHandleA(PCSTR(ptr::null())) else {
           ExitProcess(228);
           unreachable!()
       };

       let base: usize = h_base.0 as usize;

       let func_addr: usize = (base + offsets::roblox_offsets::PRINT_ADDR);
       let roblox_print: PRINT_FN = mem::transmute(func_addr);
       let c_msg = CString::new(msg).unwrap();

       roblox_print(format, c_msg.as_ptr());
   }
}

