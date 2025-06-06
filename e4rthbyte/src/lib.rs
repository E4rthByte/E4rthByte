use rbxapi::api::{print, rbx_getscheduler};

#[unsafe(no_mangle)]
pub extern "C" fn entrypoint() {
    print(0, "FEMBOY GANG");
    
    let scheduler = rbx_getscheduler();
    
    print(0, scheduler.to_string().as_str());
}