use std::io::Write;
use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

fn main() -> std::io::Result<()> {
    let (host, port) = ("localhost", 8080);
    let addr = format!("{host}:{port}");
    let listener = TcpListener::bind(addr)?;
    for stream in listener.incoming() {
        handle_client(stream?)?;
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buf = [0; 1024];
    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        stream.write_all(&buf[..bytes_read])?;
    }
}
