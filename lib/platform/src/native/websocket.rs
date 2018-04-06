use failure::Error;
use std::mem;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use ws;

type Message = Vec<u8>;

struct WebsocketInner {
    sender: RwLock<Option<ws::Sender>>,
    open: RwLock<bool>,
    incoming: Mutex<Vec<Message>>,
}

impl WebsocketInner {}

#[derive(Clone)]
pub struct Websocket(Arc<WebsocketInner>);

impl Websocket {
    fn new() -> Result<Self, Error> {
        Ok(Websocket(Arc::new(WebsocketInner {
            sender: RwLock::new(None),
            open: RwLock::new(false),
            incoming: Mutex::new(Vec::new()),
        })))
    }

    pub fn connect(url: &str) -> Result<Self, Error> {
        let client = Websocket::new()?;
        let url = String::from(url);
        {
            let client = client.clone();
            thread::spawn(move || {
                ws::connect(url, move |out| {
                    client.set_sender(out);
                    client.clone()
                }).unwrap();
            });
        }
        Ok(client)
    }

    fn set_sender(&self, sender: ws::Sender) {
        *self.0.sender.write().unwrap() = Some(sender);
    }

    pub fn open(&self) -> bool {
        *self.0.open.read().unwrap()
    }

    pub fn send(&mut self, data: Vec<u8>) -> Result<(), Error> {
        if let Some(ref sender) = *self.0.sender.read().unwrap() {
            sender.send(ws::Message::Binary(data))?;
        } else {
            return Err(format_err!("cannot send from uninitialized Websocket"));
        }
        Ok(())
    }

    pub fn incoming(&mut self) -> Result<Vec<Message>, Error> {
        Ok(mem::replace(
            &mut *self.0.incoming.lock().unwrap(),
            Vec::new(),
        ))
    }
}

impl ws::Handler for Websocket {
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        *self.0.open.write().unwrap() = true;
        Ok(())
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        let mut incoming = self.0.incoming.lock().unwrap();
        match msg {
            ws::Message::Binary(data) => incoming.push(data),
            ws::Message::Text(str_msg) => incoming.push(str_msg.as_bytes().to_vec()),
        }
        Ok(())
    }

    fn on_close(&mut self, _code: ws::CloseCode, _reason: &str) {
        *self.0.open.write().unwrap() = false;
    }

    fn on_error(&mut self, _err: ws::Error) {
        *self.0.open.write().unwrap() = false;
    }
}
