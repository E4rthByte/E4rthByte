mod rbxapi;

#[unsafe(no_mangle)]
pub extern "C" fn entrypoint() {
    rbxapi::print::print(0, "FEMBOY GANG!");
}