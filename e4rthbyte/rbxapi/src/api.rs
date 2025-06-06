use std::ffi::{c_char, c_double, CString};
use offsets::roblox::{GETTASKSCHEDULER_ADDR, PRINT_ADDR};
use crate::api;

api!(
    rbx_getscheduler, 
    GETTASKSCHEDULER_ADDR, 
    fn() -> c_double
);

api!(
    print,
    PRINT_ADDR,
    fn(format: u32, msg: &str) -> i64;
    signature: fn(u32, *const c_char, ...);
    call: |func| {
        let c_msg = CString::new(msg).unwrap();
        func(format, c_msg.as_ptr())
    }
);