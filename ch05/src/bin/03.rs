use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let (host, port) = ("localhost", 8080);
    let addr = format!("{}:{}", host, port);
    let listener = TcpListener::bind(&addr).await?;
    println!("listen at {}", addr);
    loop {
        let (mut stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            loop {
                let n = stream.read(&mut buf).await?;
                if n == 0 {
                    return Ok::<(), tokio::io::Error>(());
                }

                let s = std::str::from_utf8(&buf[..n]).unwrap();
                println!("received: {}", s);
            }
        });
    }
}
