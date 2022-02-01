use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream };
use std::thread::{ JoinHandle };

pub struct SimpleWebServer {
    workers: Vec<JoinHandle<()>>,
    listener: TcpListener
}

impl SimpleWebServer {
    pub fn new() -> Self {
        let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 80);
    
        let listener = TcpListener::bind(socket).unwrap();
        
        SimpleWebServer {
            workers: Vec::new(),
            listener: listener,
        }
    }
    
    pub fn start(&self) -> std::io::Result<()> {
        for stream in self.listener.incoming() {
            self.handle_client(stream?);
        }
    
        Ok(())
    }

    fn handle_client(&self, stream: TcpStream) {
        println!("Test!");
    }
}