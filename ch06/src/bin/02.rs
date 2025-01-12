use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};

fn main() {
    let ip_address = "127.0.0.1".parse::<Ipv4Addr>().unwrap();
    let subnet_mask = "255.255.255.0".parse::<Ipv4Addr>().unwrap();
    let gateway_address = "192.168.1.254".parse::<Ipv4Addr>().unwrap();
    let port = 8080;
    let socket_addr = SocketAddrV4::new(ip_address, port);
    let listener = TcpListener::bind(socket_addr).unwrap();
    println!("IP address: {}", ip_address);
    println!("Subnet mask: {}", subnet_mask);
    println!("Gateway address: {}", gateway_address);
    println!("Listening on: {}", listener.local_addr().unwrap());
}
