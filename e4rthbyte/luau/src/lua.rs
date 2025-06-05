use std::ffi::CString;
use std::os::raw::c_int;
use crate::raw::lua::{lua_State, lua_pushcclosurek, lua_setfield, LUA_GLOBALSINDEX};

pub fn lua_pushcfunction<S>(
    state: *mut lua_State, 
    func: unsafe extern "C" fn(*mut lua_State) -> c_int,
    debug_name: S
) 
where 
    S: AsRef<str>
{
    let debug_name = CString::new(debug_name.as_ref()).unwrap();
    
    unsafe {
        lua_pushcclosurek(
            state,
            Some(func),
            debug_name.as_ptr(),
            0,
            None
        )
    }
}

pub fn lua_setglobal<S>(
    state: *mut lua_State,
    name: S
)
where 
    S: AsRef<str>
{
    let name = CString::new(name.as_ref()).unwrap();
    
    unsafe {
        lua_setfield(
            state,
            LUA_GLOBALSINDEX,
            name.as_ptr()
        )
    }
}

pub fn register_func<S>(
    state: *mut lua_State,
    func: unsafe extern "C" fn(*mut lua_State) -> c_int,
    name: S
)
where
    S: AsRef<str>
{
    lua_pushcfunction(state, func, name.as_ref());
    lua_setglobal(state, name.as_ref());
}