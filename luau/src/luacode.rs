use std::ffi::CString;
use std::mem::zeroed;
use crate::raw::luacode::{lua_CompileOptions, luau_compile as raw_luau_compile};

/// Compiles Lua source code into Luau bytecode using the underlying FFI binding to `luau_compile`.
///
/// This function acts as a safe wrapper around the FFI `luau_compile` function.
/// It takes a string-like input 
/// representing the Lua source code and attempts to compile it into Luau bytecode.
///
/// # Parameters
///
/// - `source`: A string with the Lua source code to compile.
///
/// # Returns
///
/// - `Ok(Vec<u8>)` containing the compiled Luau bytecode on success.
/// - `Err(String)` containing the compilation error message on failure.
///
/// # Example
///
/// ```rust
/// use luau::luacode::luau_compile;
/// 
/// let source = "print('Hello, world!')";
/// assert!(luau_compile(source).is_ok());
/// 
/// let source = "print('syntax erro";
/// assert!(luau_compile(source).is_err());
/// ```
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
        if out_size > 0 && out[0] == 0 && let Ok(err) = String::from_utf8(out.clone()) {
            Err(err)
        } else {
            Ok(out)
        }
    }
}