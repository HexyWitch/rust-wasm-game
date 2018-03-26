use wasm_bindgen::prelude::*;

pub fn bootstrap() {
    use super::eval;
    eval(include_str!("../js/websocket.js"));
}

#[wasm_bindgen]
extern "C" {
    pub type WebSocket;

    #[wasm_bindgen(method)]
    pub fn send(this: &WebSocket, data: Vec<u8>);
    #[wasm_bindgen(method)]
    pub fn close(this: &WebSocket, code: i32, reason: &str);
}

#[wasm_bindgen]
extern "C" {
    pub fn websocket_connect(url: &str, event_handler: EventHandler) -> WebSocket;
}

#[wasm_bindgen]
pub struct EventHandler {
    on_message: Option<Box<FnMut(Vec<u8>) + 'static>>,
    on_open: Option<Box<FnMut() + 'static>>,
    on_close: Option<Box<FnMut() + 'static>>,
    on_error: Option<Box<FnMut() + 'static>>,
}

#[wasm_bindgen]
impl EventHandler {
    pub fn on_message(&mut self, data: Vec<u8>) {
        if let Some(ref mut on_message) = self.on_message {
            (*on_message)(data);
        }
    }
    pub fn on_open(&mut self) {
        if let Some(ref mut on_open) = self.on_open {
            (*on_open)();
        }
    }
    pub fn on_close(&mut self) {
        if let Some(ref mut on_close) = self.on_close {
            (*on_close)();
        }
    }
    pub fn on_error(&mut self) {
        if let Some(ref mut on_error) = self.on_error {
            (*on_error)();
        }
    }
}

impl EventHandler {
    pub fn new() -> EventHandler {
        EventHandler {
            on_message: None,
            on_open: None,
            on_error: None,
            on_close: None,
        }
    }
    pub fn set_on_message<T: FnMut(Vec<u8>) + 'static>(&mut self, f: T) {
        self.on_message = Some(Box::new(f));
    }
    pub fn set_on_open<T: FnMut() + 'static>(&mut self, f: T) {
        self.on_open = Some(Box::new(f));
    }
    pub fn set_on_error<T: FnMut() + 'static>(&mut self, f: T) {
        self.on_error = Some(Box::new(f));
    }
    pub fn set_on_close<T: FnMut() + 'static>(&mut self, f: T) {
        self.on_close = Some(Box::new(f));
    }
}
