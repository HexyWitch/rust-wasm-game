use failure::Error;

pub type Message = Vec<u8>;
pub trait WebSocket {
    fn connect(url: &str) -> Result<Self, Error>
    where
        Self: Sized;
    fn open(&self) -> bool;
    fn send(&mut self, msg: Vec<u8>) -> Result<(), Error>;
    fn incoming(&mut self) -> Result<Vec<Message>, Error>;
}
