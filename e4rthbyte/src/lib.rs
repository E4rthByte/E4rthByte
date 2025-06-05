mod rbxapi;

use std::ptr::null_mut;

#[unsafe(no_mangle)]
pub extern "C" fn entrypoint() {
    unsafe {
        rbxapi::print(0, "FEMBOY GANG!");
    }
}