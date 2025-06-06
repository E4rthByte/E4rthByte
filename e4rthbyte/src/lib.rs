use rbxapi::getscheduler::rbx_getscheduler;

#[unsafe(no_mangle)]
pub extern "C" fn entrypoint() {
    rbxapi::print::print(0, "FEMBOY GANG");
    
    let scheduler = rbx_getscheduler();
    
    rbxapi::print::print(0, scheduler.to_string().as_str());
}