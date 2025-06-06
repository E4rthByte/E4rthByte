use windows::core::PCSTR;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;

pub mod api;

/// Creating bindings to Roblox functions through a Rust wrapper.
///
/// #### Simple Function
/// ```_rust
/// use rbxapi::api;
/// use offsets::roblox::{GETTASKSCHEDULER_ADDR, PRINT_ADDR};
/// use std::ffi::{c_char, c_double, CString};
///
/// api!(
///     rbx_getscheduler,      // name of the wrapper function can be anything
///     GETTASKSCHEDULER_ADDR, // address of the Roblox C function, it's an usize
///     fn() -> c_double       // the function takes nothing and returns a c_double
/// )
/// ```
/// This will create a Rust function named `rbx_getscheduler`, which takes no arguments and returns a `c_double`.
///
/// #### Function with Data Transformation
/// Sometimes, you need different input parameters in the Rust function than those in the actual C function.
/// For this, the `api!` macro accepts two more arguments: `signature` and `call`.
/// ```_rust
/// api!(
///     ...
///     fn(format: u32, msg: &str) -> i64;
///     signature: fn(u32, *const c_char, ...);
///     call: |func| {
///         let c_msg = CString::new(msg).unwrap();
///         func(format, c_msg.as_ptr())
///     }
/// );
/// ```
/// In the `...`, everything remains the same — the name and the address.
/// But now `fn(...) ->` refers to the Rust wrapper function.
/// In `signature`, you specify the actual C function's signature — the parameters it takes.
///
/// Note: The return type must still match between the Rust wrapper and the C function. In the example above, it's `i64`.
///
/// The `call` block receives just one argument: the actual C function.
/// This function should be called within the `call` block with the appropriate parameters as specified in the `signature`.
/// In the example above: `u32`, `*const c_char`, `...`
///
/// Inside the `call` block, you should eventually call the C function or return whatever is needed.
#[macro_export]
macro_rules! api {
    (
        $name:ident,
        $addr:expr,
        fn($($arg:ident : $arg_ty:ty),*) -> $ret:ty
    ) => {
        pub fn $name($($arg: $arg_ty),*) -> $ret {
            unsafe {
                let func_addr = $crate:resolve_func_addrr(stringify!($name), $addr);
                let func: unsafe extern "cdecl" fn($($arg_ty),*) -> $ret = ::core::mem::transmute(func_addr);
                func($($arg),*)
            }
        }
    };

    (
        $name:ident,
        $addr:expr,
        fn($($outer_arg:ident : $outer_ty:ty),*) -> $ret:ty;
        signature: fn($($arg_ty:ty),*);
        call: |$func:ident| $call_block:block
    ) => {
        pub fn $name($($outer_arg: $outer_ty),*) -> $ret {
            unsafe {
                let func_addr = $crate:resolve_func_addrr(stringify!($name), $addr);
                let $func: unsafe extern "cdecl" fn($($arg_ty),*) -> $ret = ::core::mem::transmute(func_addr);

                $call_block
            }
        }
    };

    (
        $name:ident,
        $addr:expr,
        fn($($outer_arg:ident : $outer_ty:ty),*) -> $ret:ty;
        signature: fn($($arg_ty:ty),*, ...);
        call: |$func:ident| $call_block:block
    ) => {
        pub fn $name($($outer_arg: $outer_ty),*) -> $ret {
            unsafe {
                let func_addr = $crate:resolve_func_addrr(stringify!($name), $addr);
                let $func: unsafe extern "cdecl" fn($($arg_ty),*, ...) -> $ret = ::core::mem::transmute(func_addr);

                $call_block
            }
        }
    };

}

pub fn resolve_addr(name: &str, addr: usize) -> usize {
    unsafe {
        let Ok(base) = GetModuleHandleA(PCSTR(::core::ptr::null())) else {
            panic!(
                "Failed to get base handle for {} (0x{:X})",
                name,
                addr
            );
        };

        let base = base.0 as usize;
        base + addr
    }
}