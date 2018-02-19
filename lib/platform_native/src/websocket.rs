use platform::websocket::{Message, WebSocket};

use failure::Error;

pub struct NativeWebSocket();

impl WebSocket for NativeWebSocket {
    fn connect(_: &str) -> Result<Self, Error> {
        Ok(NativeWebSocket())
    }

    fn open(&self) -> bool {
        false
    }

    fn send(&self, msg: &str) -> Result<(), Error> {
        Ok(())
    }

    fn next<'a>(&'a mut self) -> Option<Message> {
        None
    }
}
