use std::collections::HashMap;
use std::net::{SocketAddr, TcpStream};
use std::thread;
use crate::server::listen_and_accept;

mod server;

fn main() {
    let address = "127.0.0.1:7878";
    let clients: HashMap<SocketAddr, TcpStream> = HashMap::new();

    thread::spawn(move || {
        listen_and_accept(address, clients);
    }).join().unwrap();
}
