use std::net::TcpStream;

pub trait Writer {
    fn write(&mut self, stream: &mut TcpStream);
}