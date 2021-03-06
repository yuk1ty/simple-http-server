extern crate lazy_static;
extern crate regex;

use std::net::TcpStream;
use request::Request;

use std::io::Read;
use regex::Regex;

pub struct RequestParser {
    stream: TcpStream
}

impl RequestParser {
    pub fn new(stream: &TcpStream) -> Self {
        RequestParser {
            stream: stream.try_clone().unwrap(),
        }
    }

    pub fn parse(&mut self) -> Request {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"(.*) /(.*) HTTP/(.*)\r\n").unwrap();
        }

        let mut buf = [0; 512];
        self.stream.read(&mut buf).expect("parsing error in stream");
        let result: String = String::from_utf8(buf.to_vec()).unwrap();
        let caps = REGEX.captures(&result).unwrap();

        Request::builder()
            .method(caps[1].to_string())
            .path(caps[2].to_string())
            .http_version(caps[3].to_string())
            .build()
    }
}
