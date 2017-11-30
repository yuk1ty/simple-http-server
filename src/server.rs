use std::net::TcpStream;
use request_handler::RequestHandler;
use request_parser::RequestParser;
use worker::Worker;
use writer::Writer;

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
        let req = &self.parser.from(&mut self.stream);
        &self.handler.handle(req).write(&mut self.stream);
    }
}
