mod protos;
mod server;

fn main() {
    let mut server = server::HopeShipServer::new("0.0.0.0:8080".to_string());
    server.start();
}