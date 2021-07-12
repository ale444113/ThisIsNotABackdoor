use std::process::{Command, Output};
use std::env;

pub fn execute_command(command: &str) -> Result<Output, std::io::Error>{
    match env::consts::OS{
        "windows"=>{ Command::new("cmd").args(vec!["/c", command]).output() }
        "linux" | "macos" => { Command::new("sh").args(vec!["-c",command]).output() },
        _ => { Err(std::io::Error::from(std::io::ErrorKind::NotFound)) } 
    }
}