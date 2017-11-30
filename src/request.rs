pub struct Request {
    pub head: Parts,
}

pub struct Parts {
    pub method: String,
    pub path: String,
    pub http_version: String,
}

impl Parts {
    fn new() -> Self {
        Parts {
            method: String::new(),
            path: String::new(),
            http_version: String::new(),
        }
    }
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
    pub fn new() -> Self {
        Builder::default()
    }

    pub fn method(&mut self, method: String) -> &mut Builder {
        if let Some(parts) = head(&mut self.head) {
            parts.method = method
        }
        self
    }

    pub fn path(&mut self, uri: String) -> &mut Builder {
        if let Some(parts) = head(&mut self.head) {
            parts.path = uri
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
        Builder { head: Some(Parts::new()) }
    }
}

fn head<'a>(head: &'a mut Option<Parts>) -> Option<&'a mut Parts> {
    head.as_mut()
}
