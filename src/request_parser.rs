use std::net::TcpStream;
use request::Request;

use std::io::Read;

pub struct RequestParser {

}

// 下記のようなものをパースする
// "GET / HTTP/1.1\r\n
// Host: localhost:8888\r\nConnection: keep-alive\r\nCache-Control: max-age=0\r\n
// User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/62.0.3202.94 Safari/537.36\r\n
// Upgrade-Insecure-Requests: 1\r\n
// Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8\r\n
// Accept-Encoding: gzip, deflate, br\r\n
// Accept-Language: ja,en-US;q=0.9,en;q=0.8\r\n\r\n"


impl RequestParser {
    pub fn new() -> Self {
        RequestParser {}
    }

    pub fn from(&mut self, stream: &mut TcpStream) -> Request {
        let parsed = &mut String::new();
        stream.read_to_string(parsed).unwrap();

        println!("{:?}", parsed);

        Request::builder().build()
    }
}