mod server;

fn main() {
    let server = server::SimpleWebServer::new();

    let _ = server.start();
}