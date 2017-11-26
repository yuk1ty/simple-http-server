pub struct Request {
    pub head: Parts,
}

pub struct Parts {
    pub method: String,
    pub uri: String,
    pub http_version: String,
}

impl Request {
    pub fn builder() -> Builder {
        Builder::new()
    }
}

pub struct Builder {
    head: Option<Parts>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder::default()
    }

    pub fn method(&mut self, method: String) -> &mut Builder {
        if let Some(parts) = head(&mut self.head) {
            parts.method = method
        }
        self
    }

    pub fn uri(&mut self, uri: String) -> &mut Builder {
        if let Some(parts) = head(&mut self.head) {
            parts.uri = uri
        }
        self
    }

    pub fn http_version(&mut self, http_version: String) -> &mut Builder {
        if let Some(parts) = head(&mut self.head) {
            parts.http_version = http_version
        }
        self
    }

    pub fn build(&mut self) -> Request {
        Request { head: self.take_parts() }
    }

    fn take_parts(&mut self) -> Parts {
        self.head.take().expect("Request parts unwrapping failed.")
    }
}

impl Default for Builder {
    fn default() -> Builder {
        Builder { head: None }
    }
}

fn head<'a>(head: &'a mut Option<Parts>) -> Option<&'a mut Parts> {
    head.as_mut()
}
