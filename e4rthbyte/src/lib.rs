use rbxapi::api::{print, rbx_getscheduler};

use std::panic;
use panichook::messagebox_panic_hook;

fn main() {
    print(0, "FEMBOY GANG");

    let scheduler = rbx_getscheduler();

    print(0, scheduler.to_string().as_str());
}

#[unsafe(no_mangle)]
pub extern "C" fn entrypoint() {
    panic::set_hook(Box::new(|panic_info| messagebox_panic_hook(panic_info)));
    main();
}