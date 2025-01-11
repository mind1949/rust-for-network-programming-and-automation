use std::net::ToSocketAddrs;

fn main() -> std::io::Result<()> {
    if let Ok(addrs) = "rust-lang.org:443".to_socket_addrs() {
        for addr in addrs {
            println!("{}", addr);
        }
    } else {
        println!("Failed to resolve the host.");
    }
    Ok(())
}
