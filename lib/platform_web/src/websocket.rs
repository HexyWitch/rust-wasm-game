use std::rc::Rc;
use std::cell::RefCell;
use std::any::Any;
use std::collections::VecDeque;
use failure::Error;

use js::websocket;
use js::websocket::SocketId;
use platform::websocket::{Message, WebSocket};

pub struct JsWebSocket {
    handle: SocketId,
    open: Rc<RefCell<bool>>,
    incoming: Rc<RefCell<VecDeque<Message>>>,

    onopen_handle: Box<Any>,
    onmessage_handle: Box<Any>,
}

impl Drop for JsWebSocket {
    fn drop(&mut self) {
        websocket::websocket_close(self.handle, 1000, "WebSocket dropped");
    }
}

struct MessageIter<'a>(&'a RefCell<VecDeque<Message>>);
impl<'a> Iterator for MessageIter<'a> {
    type Item = Message;
    fn next(&mut self) -> Option<Message> {
        self.0.borrow_mut().pop_front()
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

        let incoming = Rc::new(RefCell::new(VecDeque::new()));
        let incoming_cb = Rc::clone(&incoming);
        let onmessage_handle =
            websocket::websocket_onmessage(handle, move |msg| incoming_cb.borrow_mut().push_back(msg));

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

    fn send(&self, msg: &[u8]) -> Result<(), Error> {
        let open = *self.open.borrow();
        if !open {
            return Err(format_err!("error trying to send on unopened socket"));
        }
        websocket::websocket_send(self.handle, msg);
        Ok(())
    }

    fn incoming<'a>(&'a mut self) -> Box<Iterator<Item = Message> + 'a> {
        Box::new(MessageIter(&self.incoming))
    }
}
