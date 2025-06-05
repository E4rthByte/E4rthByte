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
    
    if !e4rthbyte_dll_path.exists() {
        panic!("e4rthbyte.dll not found in {}", e4rthbyte_dll_path.display());
    }
    
    println!("cargo:rerun-if-changed={}", e4rthbyte_dll_path.to_str().unwrap());
    
    let output = out_dir.join("e4rthbyte_shellcode.bin");
    
    let status = Command::new("build/donut.exe")
        .args([
            "-i", e4rthbyte_dll_path.to_str().unwrap(),
            "-o", output.to_str().unwrap(),
            "-a", "2",
            "-f", "1",
            "-m", "entrypoint"
        ])
        .status()
        .expect("Failed to run donut.exe build");
    
    if !status.success() {
        panic!("build/donut.exe failed with exit status {}", status);
    }
}