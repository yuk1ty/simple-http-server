use response::Response;

pub trait Writer {
    fn write(&mut self, res: Response);
}