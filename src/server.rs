use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream };
use std::thread::{ JoinHandle };
use std::io::{ Read, Write, BufReader, BufRead };
use std::str::from_utf8;
use std::cell::RefCell;

pub struct SimpleWebServer {
    workers: RefCell<Vec<JoinHandle<()>>>,
    socket: SocketAddr
}

impl SimpleWebServer {
    pub fn new() -> Self {
        let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 80);
        
        SimpleWebServer {
            workers: RefCell::new(Vec::new()),
            socket: socket
        }
    }
    
    pub fn start(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(self.socket).unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(_) => {
                    self.handle_connection(stream.unwrap());
                },
                Err (e) => println!("Failed to receive connection: {}", e)
            }
        }
    
        Ok(())
    }

    fn handle_connection(&self, stream: TcpStream) {
        let mut buffer: String = String::new();
        let mut reader = BufReader::new(stream);
        
        self.workers.borrow_mut().push(std::thread::spawn(move || loop {
            match reader.read_line(&mut buffer) {
                Ok(result) => {
                    if result == 0 {
                        break;
                    } else {
                        println!("{}", buffer);
                        buffer.clear();
                    }
                },
                Err (e) => {
                    println!("Failed to receive data: {}", e);
                    break;
                }
            }
        }));
    }
}