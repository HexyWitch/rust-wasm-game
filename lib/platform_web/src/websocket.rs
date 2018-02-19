use std::rc::Rc;
use std::cell::RefCell;
use std::any::Any;
use js::websocket;
use js::websocket::SocketId;

use platform::websocket::{Message, WebSocket};

pub struct JsWebSocket {
    handle: SocketId,
    open: Rc<RefCell<bool>>,
    incoming: Rc<RefCell<Vec<Message>>>,

    onopen_handle: Box<Any>,
    onmessage_handle: Box<Any>,
}

impl Drop for JsWebSocket {
    fn drop(&mut self) {
        websocket::websocket_close(self.handle, 1000, "WebSocket dropped");
    }
}

impl WebSocket for JsWebSocket {
    fn connect(url: &str) -> Result<Self, Error> {
        let handle = websocket::websocket_create(url);

        let open = Rc::new(RefCell::new(false));
        let open_cb = Rc::clone(&open);
        let onopen_handle = unsafe {
            websocket::websocket_onopen(handle, move || {
                *open_cb.borrow_mut() = true;
            })
        };

        let incoming = Rc::new(RefCell::new(Vec::new()));
        let incoming_cb = Rc::clone(&incoming);
        let onmessage_handle = unsafe {
            websocket::websocket_onmessage(handle, move |msg| {
                incoming_cb.borrow_mut().push(String::from(msg))
            })
        };

        Ok(JsWebSocket {
            handle: handle,
            open: open,
            incoming: incoming,

            onopen_handle: Box::new(onopen_handle),
            onmessage_handle: Box::new(onmessage_handle),
        })
    }

    fn open(&self) -> bool {
        *self.open.borrow()
    }

    fn send(&self, msg: &str) -> Result<(), Error> {
        websocket::websocket_send(self.handle, msg);
        Ok(())
    }

    fn next<'a>(&'a mut self) -> Option<Message> {
        let mut incoming = self.incoming.borrow_mut();
        let v = incoming.drain(0..).next();
        v
    }
}
