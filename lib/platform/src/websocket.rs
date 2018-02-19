pub type Message = String;
pub trait WebSocket {
    fn connect(url: &str) -> Result<Self, ()>
    where
        Self: Sized;
    fn open(&self) -> bool;
    fn send(&self, msg: &str) -> Result<(), ()>;
    fn next<'a>(&'a mut self) -> Option<Message>;
}
