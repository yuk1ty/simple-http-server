use status::StatusCode;
use writer::Writer;
use mime::Mime;

use std::net::TcpStream;
use std::io::Write;

pub struct Response {
    head: Parts,
    body: String, // ライフタイムパラメータをつけるのがめんどくさいので端折りました
}

pub struct Parts {
    pub status: String,
    pub content_type: String,
    pub content_length: String,
}

struct Body {
    content: String,
}

pub struct Builder {
    head: Option<Parts>,
    body: Option<Body>,
}

impl Response {
    pub fn builder() -> Builder {
        Builder::new()
    }
}

impl Builder {
    pub fn new() -> Builder {
        Builder::default()
    }

    pub fn status(&mut self, status: StatusCode) -> &mut Builder {
        if let Some(parts) = head(&mut self.head) {
            parts.status = status.to_string()
        }
        self
    }

    pub fn content_type(&mut self, content_type: Mime) -> &mut Builder {
        if let Some(parts) = head(&mut self.head) {
            parts.content_type = content_type.to_string()
        }
        self
    }

    pub fn content_length(&mut self, content_length: String) -> &mut Builder {
        if let Some(parts) = head(&mut self.head) {
            parts.content_length = content_length
        }
        self
    }

    pub fn body(&mut self, content: String) -> &mut Builder {
        if let Some(body) = mut_body(&mut self.body) {
            body.content = content
        }
        self
    }

    fn take_parts(&mut self) -> Parts {
        self.head.take().expect("Response parts unwrapping failed.")
    }

    fn take_body(&mut self) -> Body {
        self.body.take().expect("Response body unwrapping failed.")
    }

    pub fn build(&mut self) -> Response {
        Response {
            head: self.take_parts(),
            body: self.take_body().content,
        }
    }
}

impl Default for Builder {
    fn default() -> Builder {
        Builder {
            head: None,
            body: None,
        }
    }
}

fn head<'a>(head: &'a mut Option<Parts>) -> Option<&'a mut Parts> {
    head.as_mut()
}

fn mut_body<'a>(body: &'a mut Option<Body>) -> Option<&'a mut Body> {
    body.as_mut()
}

impl Writer for Response {
    fn write(&mut self, stream: &mut TcpStream) {
        let response = r"HTTP/1.1";
        let response_bytes = response.as_bytes();
        stream.write(response_bytes).unwrap();
        stream.write(self.body.as_bytes()).unwrap();
        stream.flush().expect("Response flush failed.");
    }
}
