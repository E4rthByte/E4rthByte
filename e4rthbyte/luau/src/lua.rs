#[macro_export]
macro_rules! lua_pushcfunction {
    ($state:expr, $func:expr) => {
        unsafe {
            $crate::raw::lua::lua_pushcclosurek(
                $state,
                Some(lua_cfunction!($func)),
                core::ptr::null_mut(),
                0,
                None
            )
        }
    }
}

#[macro_export]
macro_rules! lua_setglobal {
    ($state:expr, $name:expr) => {
        unsafe {
            $crate::raw::lua::lua_setfield(
                $state,
                $crate::raw::lua::LUA_GLOBALSINDEX,
                std::ffi::CString::new($name).unwrap().as_ptr()
            )
        }
    };
}

#[macro_export]
macro_rules! lua_cfunction {
    ($func:expr) => {{
        $func as extern "C" fn(*mut $crate::raw::lua::lua_State) -> core::ffi::c_int
    }};
}

#[macro_export]
macro_rules! lua_registercfunction {
    ($state:expr, $name:expr, $func:expr) => {
        $crate::lua_pushcfunction!($state, $func);
        $crate::lua_setglobal!($state, $name);
    };
}
