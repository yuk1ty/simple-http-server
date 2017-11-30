use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use request::Request;
use response::Response;
use status::StatusCode;
use mime::Mime;

pub struct RequestHandler;

impl RequestHandler {
    pub fn new() -> Self {
        RequestHandler
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
