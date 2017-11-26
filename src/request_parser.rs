extern crate lazy_static;
extern crate regex;

use std::net::TcpStream;
use request::Request;

use std::io::Read;
use regex::Regex;

pub struct RequestParser {}

impl RequestParser {
    pub fn new() -> Self {
        RequestParser {}
    }

    pub fn from(&mut self, stream: &mut TcpStream) -> Request {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"(.*) / HTTP/(.*)\r\nHost: (.*)\r\n").unwrap();
        }

        let parsed = &mut String::new();
        stream.read_to_string(parsed).unwrap();
        let caps = REGEX.captures(parsed).unwrap();

        Request::builder()
            .method(caps[1].to_string())
            .http_version(caps[2].to_string())
            .uri(caps[3].to_string())
            .build()
    }
}
