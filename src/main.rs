use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream };

fn main() -> std::io::Result<()> {
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 80);

    let listener = TcpListener::bind(socket).unwrap();

    for stream in listener.incoming() {
        handle_client(stream?);
    }

    Ok(())
}

fn handle_client(stream: TcpStream) {
    println!("Test!");
}