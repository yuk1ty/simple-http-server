extern crate simple_http_server;

use simple_http_server::request_handler::RequestHandler;
use simple_http_server::request_parser::RequestParser;
use simple_http_server::server::Server;
use simple_http_server::ThreadPool;

use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};

fn main() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8888);
    let listener = TcpListener::bind(addr).unwrap();

    let pool = ThreadPool::new(4);
    for socket in listener.incoming() {
        let socket = socket.unwrap();
        pool.execute(move || {
            let mut server = Server::new(RequestHandler::new(&socket), RequestParser::new(&socket));
            server.start();
        });
    }
}
