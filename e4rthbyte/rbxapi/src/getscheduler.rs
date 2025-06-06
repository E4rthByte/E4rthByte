use std::{mem, ptr};
use std::ffi::c_double;
use windows::core::PCSTR;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::System::Threading::ExitProcess;
use offsets::roblox::GETTASKSCHEDULER_ADDR;

type RbxGetSchedulerFn = unsafe extern "cdecl" fn() -> c_double;
pub fn rbx_getscheduler() -> c_double {
    unsafe {
        let Ok(h_base) = GetModuleHandleA(PCSTR(ptr::null())) else {
            ExitProcess(228);
        };

        let base: usize = h_base.0 as usize;
        
        let func_addr: usize = base + GETTASKSCHEDULER_ADDR;
        let getscheduler: RbxGetSchedulerFn = mem::transmute(func_addr);
        
        getscheduler()
    }
}