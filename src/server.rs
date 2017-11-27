use std::net::TcpStream;
use request_handler::RequestHandler;
use request_parser::RequestParser;
use response::*;
use worker::Worker;
use writer::Writer;
use std::io::Write;

pub struct Server {
    stream: TcpStream,
    handler: RequestHandler,
    parser: RequestParser,
}

impl Server {
    pub fn new(stream: TcpStream, handler: RequestHandler, parser: RequestParser) -> Self {
        Server {
            stream: stream,
            handler: handler,
            parser: parser,
        }
    }
}

impl Worker for Server {
    fn start(&mut self) {
        &self.parser.from(&mut self.stream);
        &self.handler.handle().write(&mut self.stream);
    }
}
