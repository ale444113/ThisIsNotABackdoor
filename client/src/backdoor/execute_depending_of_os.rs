use std::process::{Command, Output};
use std::env;
fn execute_command(command:&str){
    match env::consts::OS{
        "windows"=>{Command::new("cmd").args(vec![command]);}
        "linux"=>{Command::new("sh").args(vec!["-c",command]);},
        "macos"=>{
            Command::new("sh").args(vec!["-c",command]);
        },
        _=>{}
    }

}