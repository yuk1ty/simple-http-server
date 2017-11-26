pub enum Mime {
    Html,
    Js,
    Css,
    Jpg,
    Png,
    Gif,
}

impl Mime {
    pub fn to_string(&self) -> String {
        match *self {
            Mime::Html => String::from("text/html;charset=utf8"),
            Mime::Js => String::from("application/javascript"),
            Mime::Css => String::from("text/css"),
            Mime::Jpg => String::from("image/jpg"),
            Mime::Png => String::from("image/png"),
            Mime::Gif => String::from("image/gif"),
        }
    }
}
