use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use request::{Request, Parts};
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

    pub fn handle(&mut self) -> Response {
        match self.req.take() {
            Some(request) => {
                let status_code;
                let mime;
                let body;
                {
                    if request.head.uri.starts_with("public/") {
                        status_code = StatusCode::Forbidden;
                        mime = Mime::Html;
                        body = read_html("forbidden.html");
                    } else {
                        status_code = StatusCode::Ok;
                        mime = Mime::Html;
                        body = read_html("ok.html");
                    }
                }
                Response::builder().status(status_code).content_type(mime).body(body).build()
            },
            None => {
                Response::builder()
                    .status(StatusCode::BadRequest)
                    .content_type(Mime::Html)
                    .body(read_html("404.html"))
                    .build()
            }
        }
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
