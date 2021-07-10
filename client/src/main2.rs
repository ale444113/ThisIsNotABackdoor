use std::process::Command;
fn main(){
let mut options = std::run::ProcessOptions::new();
let process = std::run::Process::new("ls", &[your, arguments], options);
}