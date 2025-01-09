use std::io::{Read, Write};
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let port = 8080;
    let host = "localhost";
    let listener = TcpListener::bind(format!("{}:{}", host, port))?;
    println!("Listening on port: {:?}", port);
    for stream in listener.incoming() {
        let mut stream = stream?;
        println!("New client connected: {:?}", stream.peer_addr()?);

        let mut buf = [0; 1024];
        loop {
            let bytes_read = stream.read(&mut buf)?;
            if bytes_read == 0 {
                println!("Client disconnected");
                break;
            }
            stream.write_all(&buf)?;
        }
    }
    Ok(())
}
