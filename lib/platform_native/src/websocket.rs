use platform::websocket::{Message, WebSocket};

pub struct NativeWebSocket();

impl WebSocket for NativeWebSocket {
    fn connect(_: &str) -> Result<Self, ()> {
        Ok(NativeWebSocket())
    }

    fn open(&self) -> bool {
        false
    }

    fn send(&self, msg: &str) -> Result<(), ()> {
        Ok(())
    }

    fn next<'a>(&'a mut self) -> Option<Message> {
        None
    }
}
