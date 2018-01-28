type Message = String;

pub struct WebSocket {}

// temporary null implementation
impl WebSocket {
    pub fn connect(_url: &str) -> Result<WebSocket, ()> {
        Ok(WebSocket {})
    }

    pub fn open(&self) -> bool {
        false
    }

    pub fn send(&self, _msg: &str) -> Result<(), ()> {
        Ok(())
    }

    pub fn next<'a>(&'a mut self) -> Option<Message> {
        None
    }
}
