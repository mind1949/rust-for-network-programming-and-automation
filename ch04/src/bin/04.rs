use std::io::Read;
use std::net::TcpStream;

fn handle_data(_buf: &mut [u8]) {
    // process data
}

fn main() -> std::io::Result<()> {
    let (host, port) = ("localhost", 8080);
    let addr = format!("{}:{}", host, port);
    let mut stream = TcpStream::connect(addr)?;
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(n) => {
                handle_data(&mut buffer[..n]);
            }
            Err(err) => {
                println!("error: {}", err);
                break;
            }
        }
    }
    Ok(())
}
