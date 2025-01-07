use std::io::prelude::*;
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut buffer = [0;1024];
        loop {
            let bytes_read = stream.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            let message = String::from_utf8_lossy(&mut buffer);
            println!("Received Mesage: {}", message);
        }
    }
    Ok(())
}
