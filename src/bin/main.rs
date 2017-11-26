extern crate simple_http_server;

use simple_http_server::request_handler::RequestHandler;
use simple_http_server::request_parser::RequestParser;
use simple_http_server::server::Server;
use simple_http_server::worker::Worker;

use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use std::thread;

fn main() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8888);
    let listener = TcpListener::bind(addr).unwrap();

    // TODO incoming?
    match listener.accept() {
        Ok((socket, _addr)) => {
            let mut server = Server::new(socket, RequestHandler::new(), RequestParser::new());
            let handle = thread::spawn(move || { server.start(); });
            handle.join().unwrap();
        }
        Err(e) => println!("couldnt get client: {:?}", e),
    };
    println!("end");
}
