use request_handler::RequestHandler;
use request_parser::RequestParser;
use writer::Writer;

pub struct Server {
    handler: RequestHandler,
    parser: RequestParser,
}

impl Server {
    pub fn new(handler: RequestHandler, parser: RequestParser) -> Self {
        Server {
            handler: handler,
            parser: parser,
        }
    }

    pub fn start(&mut self) {
        let req = self.parser.parse();
        let res = self.handler.handle(&req);
        self.handler.write(res)
    }
}
