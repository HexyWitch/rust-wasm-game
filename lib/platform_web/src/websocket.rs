use failure::Error;
use std::cell::RefCell;
use std::rc::Rc;

use js::websocket;
use platform::websocket::WebSocket as WebsocketTrait;

type Message = Vec<u8>;

pub struct Websocket {
    js_socket: websocket::WebSocket,

    incoming: Rc<RefCell<Vec<Message>>>,
    open: Rc<RefCell<bool>>,
}

impl Drop for Websocket {
    fn drop(&mut self) {
        if self.open() {
            self.js_socket.close(1000, "WebSocket dropped");
        }
    }
}

impl WebsocketTrait for Websocket {
    fn connect(url: &str) -> Result<Self, Error> {
        let mut event_handler = websocket::EventHandler::new();

        let open = Rc::new(RefCell::new(false));
        let open_cb = Rc::clone(&open);
        event_handler.set_on_open(move || {
            *open_cb.borrow_mut() = true;
        });

        let incoming = Rc::new(RefCell::new(Vec::new()));
        let incoming_cb = Rc::clone(&incoming);
        event_handler.set_on_message(move |msg| incoming_cb.borrow_mut().push(msg));

        let js_socket = websocket::websocket_connect(url, event_handler);
        Ok(Websocket {
            js_socket,
            incoming,
            open,
        })
    }

    fn open(&self) -> bool {
        *self.open.borrow()
    }

    fn send(&mut self, data: Message) -> Result<(), Error> {
        let open = *self.open.borrow();
        if !open {
            return Err(format_err!("error trying to send on unopened socket"));
        }
        self.js_socket.send(data);
        Ok(())
    }

    fn incoming(&mut self) -> Result<Vec<Message>, Error> {
        Ok(self.incoming.replace(Vec::new()))
    }
}
