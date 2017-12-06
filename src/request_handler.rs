use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::net::TcpStream;

use request::Request;
use response::Response;
use status::StatusCode;
use mime::Mime;
use writer::Writer;

pub struct RequestHandler {
    stream: TcpStream,
}

impl RequestHandler {
    pub fn new(stream: &TcpStream) -> Self {
        RequestHandler {
            stream: stream.try_clone().unwrap()
        }
    }

    pub fn handle(&mut self, req: &Request) -> Response {
        if req.head.path.starts_with("static") {
            let forbidden = read_html("./src/static/assets/html/403.html");
            return Response::builder()
                .status(StatusCode::Forbidden)
                .content_type(Mime::Html)
                .content_length(len2str(&forbidden))
                .body(forbidden)
                .build();
        } else if req.head.path == "hello" {
            let content = read_html("./src/static/assets/html/hello.html");
            return Response::builder()
                .status(StatusCode::Ok)
                .content_type(Mime::Html)
                .content_length(len2str(&content))
                .body(content)
                .build();
        }

        let not_found = read_html("./src/static/assets/html/404.html");
        return Response::builder()
            .status(StatusCode::NotFound)
            .content_type(Mime::Html)
            .content_length(len2str(&not_found))
            .body(not_found)
            .build();
    }
}

impl Writer for RequestHandler {
    fn write(&mut self, res: Response) {
        let res_str = into_http_response(&res);
        self.stream.write(res_str.as_bytes()).unwrap();
        self.stream.write(res.body.as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }
}

fn into_http_response(res: &Response) -> String {
    format!("HTTP/1.1 {}\r\nServer: SimpleRustHttpServer\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: Close\r\n\r\n", &res.head.status, &res.head.content_type, &res.head.content_length)
}

fn len2str<'a>(content: &'a str) -> String {
    (content.len() as i32).to_string()
}

fn read_html(path: &'static str) -> String {
    let f = File::open(path).expect("failed to open file.");
    let mut lines = String::new();
    BufReader::new(f).read_to_string(&mut lines).expect(
        "failed to read file.",
    );
    lines
}
