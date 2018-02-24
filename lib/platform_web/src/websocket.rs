use std::rc::Rc;
use std::cell::RefCell;
use std::any::Any;
use failure::Error;

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
        let onopen_handle = websocket::websocket_onopen(handle, move || {
            *open_cb.borrow_mut() = true;
        });

        let incoming = Rc::new(RefCell::new(Vec::new()));
        let incoming_cb = Rc::clone(&incoming);
        let onmessage_handle =
            websocket::websocket_onmessage(handle, move |msg| incoming_cb.borrow_mut().push(msg));

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

    fn send(&mut self, data: Vec<u8>) -> Result<(), Error> {
        let open = *self.open.borrow();
        if !open {
            return Err(format_err!("error trying to send on unopened socket"));
        }
        websocket::websocket_send(self.handle, &data);
        Ok(())
    }

    fn incoming(&mut self) -> Result<Vec<Message>, Error> {
        Ok(self.incoming.replace(Vec::new()))
    }
}
