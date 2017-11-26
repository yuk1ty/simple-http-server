pub enum StatusCode {
    Ok,
    BadRequest,
    Forbidden,
    NotFound,
    InternalServerError,
}

impl StatusCode {
    pub fn to_string(&self) -> String {
        match *self {
            StatusCode::Ok => String::from("200 OK"),
            StatusCode::BadRequest => String::from("400 BadRequest"),
            StatusCode::Forbidden => String::from("403 Forbidden"),
            StatusCode::NotFound => String::from("404 Not Found"),
            StatusCode::InternalServerError => String::from("500 InternalServerError"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::StatusCode::*;

    #[test]
    fn test_status_code() {}
}
