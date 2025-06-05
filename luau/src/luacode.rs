use std::ffi::CString;
use std::mem::zeroed;
use std::slice;
use crate::raw::luacode::{lua_CompileOptions, luau_compile as raw_luau_compile};

pub fn luau_compile<S>(source: S) -> Result<Vec<u8>, String>
where
    S: AsRef<str>
{
    let source = CString::new(source.as_ref()).unwrap();
    let mut options = lua_CompileOptions {
        optimizationLevel: 1,
        debugLevel: 1,
        coverageLevel: 1,
        ..unsafe { zeroed() }
    };

    let mut out_size = 0usize;

    unsafe {
        let bytecode_ptr = raw_luau_compile(
            source.as_ptr(),
            source.as_bytes().len(),
            &mut options,
            &mut out_size,
        );

        let out = Vec::from_raw_parts(bytecode_ptr as *mut u8, out_size, out_size);
        if let Ok(err) = String::from_utf8(out.clone()) {
            Err(err)
        } else {
            Ok(out)
        }
    }
}