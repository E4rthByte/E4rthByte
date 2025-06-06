use std::panic;
use panichook::messagebox_panic_hook;

#[unsafe(no_mangle)]
pub extern "C" fn entrypoint() {
    panic::set_hook(Box::new(|panic_info| messagebox_panic_hook(panic_info)));
    
    rbxapi::print::print(0, "FEMBOY GANG!");
}