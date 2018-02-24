use failure::Error;

use platform::websocket::{Message, WebSocket};

pub struct NativeWebSocket{
    incoming: Vec<Message>
}

impl WebSocket for NativeWebSocket {
    fn connect(_: &str) -> Result<Self, Error> {
        Ok(NativeWebSocket{incoming: Vec::new()})
    }

    fn open(&self) -> bool {
        false
    }

    fn send(&self, _msg: &[u8]) -> Result<(), Error> {
        Ok(())
    }

    fn incoming<'a>(&'a mut self) -> Box<Iterator<Item = Message> + 'a> {
        Box::new(self.incoming.drain(0..))
    }
}
