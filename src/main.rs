mod server;

fn main() {
    let server = server::SimpleWebServer::new();

    server.start();
}