use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target_dir = out_dir
        .ancestors()
        .nth(3)
        .expect("Failed to get target dir");
    let e4rthbyte_dll_path = target_dir.join("e4rthbyte.dll");
    
    println!("cargo:rustc-env=E4RTHBYTE_DLL_PATH={}", e4rthbyte_dll_path.display());
}