use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target_dir = out_dir
        .ancestors()
        .nth(3)
        .expect("Failed to get target dir");
    let e4rthbyte_dll_path = target_dir.join("e4rthbyte.dll");
    let shell_code_path = target_dir.join("shellcode.bin");
    
    println!("cargo:rustc-env=E4RTHBYTE_DLL_PATH={}", e4rthbyte_dll_path.display());

    let status = Command::new("donut.exe") // add donut to path
        .args(&[
            "-i",
            &e4rthbyte_dll_path.to_string_lossy(),
            "-o",
            &shell_code_path.to_string_lossy(),
            "-a",
            "2",
            "-f",
            "1",
            "-m",
            "entrypoint",
        ])
        .status()
        .expect("Failed to run donut.exe");
    
    if !status.success() {
        panic!("Converting to shellcode failed with exit status {}", status);
    }

    println!("cargo:rustc-env=SHELL_CODE_PATH={}", shell_code_path.display());
}