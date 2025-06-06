#![feature(future_join)]

use std::future::join;
use rbxapi::api::{print, rbx_getscheduler};

use std::panic;
use std::time::Duration;
use smol::Timer;
use panichook::messagebox_panic_hook;

async fn scheduler() {
    loop {
        let scheduler = rbx_getscheduler();
        print(0, format!("scheduler: {}", scheduler).as_str());
        Timer::after(Duration::from_millis(50)).await;
    }
}

async fn print_gang() {
    loop {
        print(0, "FEMBOY GANG");
        Timer::after(Duration::from_millis(600)).await;
    }
}

async fn main() {
    join!(scheduler(), print_gang()).await;
}

#[unsafe(no_mangle)]
pub extern "C" fn entrypoint() {
    panic::set_hook(Box::new(|panic_info| messagebox_panic_hook(panic_info)));
    
    smol::block_on(async { 
        main().await
    });
}