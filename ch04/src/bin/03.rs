use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() -> std::io::Result<()> {
    let (host, port) = ("127.0.0.1", 8080);
    let addr = format!("{}:{}", host, port);
    let listener = TcpListener::bind(addr)?;
    for stream in listener.incoming() {
        let stream = stream?;
        thread::spawn(move || handle_connection(&mut stream.try_clone()?));
    }
    Ok(())
}

fn handle_connection(_stream: &mut TcpStream) -> std::io::Result<()> {
    Ok(())
}
