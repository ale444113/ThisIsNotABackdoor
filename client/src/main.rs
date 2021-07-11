use std::io::prelude::*;
use std::net::TcpStream;
use std::str::from_utf8;

use std::process::{Command, Output};

fn establish_connection(mut wait: i64) -> TcpStream {
    if wait > 1800 {
        wait = 1800
    }
    if let Ok(stream) = TcpStream::connect("localhost:5050") {
        println!("Connected to the server!");
        return stream;
    } else {
        establish_connection(wait * 2)
    }
}

fn main() -> std::io::Result<()> {
    let mut stream: TcpStream = establish_connection(10);
    loop {
        let mut data = [0; 1024];//hay que tener esto para manejar una gran cantidad de datos , con 6 bytes no basta ya que solo te da una salida de 6 caracteres
        match stream.read(&mut data) {
            Ok(_) => {
                let command = from_utf8(&data).unwrap();
                println!("{}",command);
                stream.write(command.as_bytes()).unwrap_or(0);
                /*let output = Command::new("sh")
                                    .arg("command")
                                    .output()
                                    .expect("failed to execute process");

                            println!("status: {}", output.status);
                            println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
                            println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
                */
                ()
            }
            Err(_) => (),
        }
    }
}
