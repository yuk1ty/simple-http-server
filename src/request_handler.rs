use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use request::Request;
use response::Response;
use status::StatusCode;
use mime::Mime;

pub struct RequestHandler {
    req: Option<Request>,
}

impl RequestHandler {
    pub fn new() -> Self {
        RequestHandler { req: None }
    }

    // TODO ファイル名と長ったらしいコードの修正
    pub fn handle(&mut self) -> Response {
        let content = "<html><head><title>Title</title></head><body>success</body></html>".to_string();
        Response::builder()
            .status(StatusCode::Ok)
            .content_type(Mime::Html)
            .content_length((content.len() as i32).to_string())
            .body(content)
            .build()
    }
}

fn read_html(path: &'static str) -> String {
    let f = File::open(path).expect("failed to open file.");
    let mut lines = String::new();
    BufReader::new(f).read_line(&mut lines).expect(
        "failed to read file.",
    );
    lines
}
