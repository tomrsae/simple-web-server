use core::unicode::conversions;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream };
use std::thread::{ JoinHandle };
use std::io::{ Read, Write, BufReader, BufRead };
use std::str::from_utf8;
use std::cell::{ RefCell, Cell };
use std::sync::{ Arc, Mutex, Condvar };

pub struct SimpleWebServer {
    pair: Arc<(Mutex<Workers>, Condvar)>,
    socket: SocketAddr
}

struct Workers {
    running: bool,
    connections: Vec<JoinHandle<()>>
}

impl Workers {
    fn new() -> Self {
        Workers {
            running: false,
            connections: Vec::new()
        }
    }
}

impl SimpleWebServer {
    pub fn new() -> Self {
        let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 80);

        let pair = Arc::new((Mutex::new(Workers::new()), Condvar::new()));
        
        SimpleWebServer {
            pair: pair,
            socket: socket
        }
    }
    
    pub async fn start(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(self.socket).unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(_) => {
                    self.handle_connection(stream.unwrap()).await;
                },
                Err (e) => println!("Failed to receive connection: {}", e)
            }
        }
    
        Ok(())
    }

    async fn handle_connection(&self, stream: TcpStream) {
        let mut buffer: String = String::new();
        let mut reader = BufReader::new(stream);

        let (lock, condvar) = &*self.pair;
        let mut workers = lock.lock().unwrap();
        workers = condvar.wait_while(workers, |workers| !(*workers).running).unwrap();
        
        (*workers).connections.push(std::thread::spawn(move || loop {
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