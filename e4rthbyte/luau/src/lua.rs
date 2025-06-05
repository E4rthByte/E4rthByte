use std::ffi::CString;
use std::os::raw::c_int;
use crate::raw::lua::{lua_State, lua_pushcclosurek, lua_setfield, LUA_GLOBALSINDEX};

pub fn lua_pushcfunction<S, F>(
    state: &mut Box<lua_State>, 
    func: unsafe extern "C" fn(*mut lua_State) -> c_int,
    debug_name: S
) 
where 
    S: AsRef<str>
{
    let debug_name = CString::new(debug_name.as_ref()).unwrap();
    
    unsafe {
        lua_pushcclosurek(
            state.as_mut() as _,
            Some(func),
            debug_name.as_ptr(),
            0,
            None
        )
    }
}

pub fn lua_setglobal<S>(
    state: &mut Box<lua_State>,
    name: S
)
where 
    S: AsRef<str>
{
    let name = CString::new(name.as_ref()).unwrap();
    
    unsafe {
        lua_setfield(
            state.as_mut() as _,
            LUA_GLOBALSINDEX,
            name.as_ptr()
        )
    }
}