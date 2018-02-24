use failure::Error;
use std::mem;
use std::thread;
use std::sync::{Arc, Mutex, RwLock};
use ws;

use platform::websocket::{Message, WebSocket};

struct ClientInner {
    sender: RwLock<Option<ws::Sender>>,
    open: RwLock<bool>,
    incoming: Mutex<Vec<Message>>,
    on_close: Box<Fn() + Send + Sync + 'static>,
}

impl ClientInner {}

#[derive(Clone)]
pub struct Client(Arc<ClientInner>);

impl Client {
    pub fn new<F>(on_close: F) -> Result<Self, Error>
    where
        F: Fn() + Send + Sync + 'static,
    {
        Ok(Client(Arc::new(ClientInner {
            sender: RwLock::new(None),
            open: RwLock::new(false),
            incoming: Mutex::new(Vec::new()),
            on_close: Box::new(on_close),
        })))
    }

    pub fn with_sender<F>(sender: ws::Sender, on_close: F) -> Result<Self, Error>
    where
        F: Fn() + Send + Sync + 'static,
    {
        Ok(Client(Arc::new(ClientInner {
            sender: RwLock::new(Some(sender)),
            open: RwLock::new(false),
            incoming: Mutex::new(Vec::new()),
            on_close: Box::new(on_close),
        })))
    }

    pub fn set_sender(&self, sender: ws::Sender) {
        *self.0.sender.write().unwrap() = Some(sender);
    }
}

impl ws::Handler for Client {
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
        (self.0.on_close)();
    }

    fn on_error(&mut self, _err: ws::Error) {
        *self.0.open.write().unwrap() = false;
    }
}

impl WebSocket for Client {
    fn connect(url: &str) -> Result<Self, Error> {
        let client = Client::new(|| {})?;
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

    fn open(&self) -> bool {
        *self.0.open.read().unwrap()
    }

    fn send(&mut self, data: Vec<u8>) -> Result<(), Error> {
        if let Some(ref sender) = *self.0.sender.read().unwrap() {
            sender.send(ws::Message::Binary(data))?;
        } else {
            return Err(format_err!("cannot send from uninitialized client"));
        }
        Ok(())
    }

    fn incoming(&mut self) -> Result<Vec<Message>, Error> {
        Ok(mem::replace(
            &mut *self.0.incoming.lock().unwrap(),
            Vec::new(),
        ))
    }
}
