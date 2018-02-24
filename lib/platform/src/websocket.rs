use failure::Error;

pub type Message = Vec<u8>;
pub trait WebSocket {
    fn connect(url: &str) -> Result<Self, Error>
    where
        Self: Sized;
    fn open(&self) -> bool;
    fn send(&self, msg: &[u8]) -> Result<(), Error>;
    fn incoming<'a>(&'a mut self) -> Box<Iterator<Item = Message> + 'a>;
}
