use status::StatusCode;
use mime::Mime;

pub struct Response {
    pub head: Parts,
    pub body: String, // ライフタイムパラメータをつけるのがめんどくさいので端折りました
}

pub struct Parts {
    pub status: String,
    pub content_type: String,
    pub content_length: String,
}

impl Parts {
    fn new() -> Self {
        Parts {
            status: String::new(),
            content_type: String::new(),
            content_length: String::new(),
        }
    }
}

struct Body {
    content: String,
}

impl Body {
    fn new() -> Self {
        Body { content: String::new() }
    }
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
    pub fn new() -> Self {
        Builder::default()
    }

    pub fn status(&mut self, status: StatusCode) -> &mut Builder {
        if let Some(parts) = head_mut(&mut self.head) {
            parts.status = status.to_string()
        }
        self
    }

    pub fn content_type(&mut self, content_type: Mime) -> &mut Builder {
        if let Some(parts) = head_mut(&mut self.head) {
            parts.content_type = content_type.to_string()
        }
        self
    }

    pub fn content_length(&mut self, content_length: String) -> &mut Builder {
        if let Some(parts) = head_mut(&mut self.head) {
            parts.content_length = content_length
        }
        self
    }

    pub fn body(&mut self, content: String) -> &mut Builder {
        if let Some(body) = body_mut(&mut self.body) {
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
            head: Some(Parts::new()),
            body: Some(Body::new()),
        }
    }
}

fn head_mut<'a>(head: &'a mut Option<Parts>) -> Option<&'a mut Parts> {
    head.as_mut()
}

fn body_mut<'a>(body: &'a mut Option<Body>) -> Option<&'a mut Body> {
    body.as_mut()
}