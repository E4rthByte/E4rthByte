use rbxapi::api::{print, rbx_getscheduler};

use std::{panic, thread};
use tokio::runtime::Builder;
use tokio::time::{sleep, Duration};
use panichook::messagebox_panic_hook;

async fn main() {
    print(0, "FEMBOY GANG");
    
    // loop {
        let scheduler = rbx_getscheduler();
        print(0, scheduler.to_string().as_str());
        
        // sleep(Duration::from_millis(500)).await;
    // }
}

#[unsafe(no_mangle)]
pub extern "C" fn entrypoint() {
    panic::set_hook(Box::new(|panic_info| messagebox_panic_hook(panic_info)));
    
    let rt = Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Failed to build Tokio runtime");
    
    // TODO: run it in the new thread. The std::thread::spawn causes roblox freeze and crash
    rt.block_on(main());
}