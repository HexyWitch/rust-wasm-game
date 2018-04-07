extern crate bincode;
#[macro_use]
extern crate failure;
extern crate game;
extern crate ws;

use bincode::{deserialize, serialize};
use failure::Error;
use std::collections::HashMap;
use std::mem;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

use game::GameServer;
use game::net::{ClientId, Packet};

type Message = Vec<u8>;

pub struct ClientInner {
    sender: RwLock<Option<ws::Sender>>,
    incoming: Mutex<Vec<Message>>,
    on_close: Box<Fn() + Send + Sync + 'static>,
}

#[derive(Clone)]
pub struct Client(Arc<ClientInner>);

impl Client {
    fn with_sender<F>(sender: ws::Sender, on_close: F) -> Result<Self, Error>
    where
        F: Fn() + Send + Sync + 'static,
    {
        Ok(Client(Arc::new(ClientInner {
            sender: RwLock::new(Some(sender)),
            incoming: Mutex::new(Vec::new()),
            on_close: Box::new(on_close),
        })))
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

impl ws::Handler for Client {
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        let mut incoming = self.0.incoming.lock().unwrap();
        match msg {
            ws::Message::Binary(data) => incoming.push(data),
            ws::Message::Text(str_msg) => incoming.push(str_msg.as_bytes().to_vec()),
        }
        Ok(())
    }

    fn on_close(&mut self, _code: ws::CloseCode, _reason: &str) {
        (self.0.on_close)();
    }
}

fn main() {
    let connected = Arc::new(Mutex::new(Vec::new()));
    let disconnected = Arc::new(Mutex::new(Vec::new()));
    {
        let mut next_id = 0;
        let connected = Arc::clone(&connected);
        let disconnected = Arc::clone(&disconnected);
        thread::spawn(move || {
            ws::listen("127.0.0.1:2794", |out| {
                let client_id = next_id;
                next_id += 1;

                let disconnected = Arc::clone(&disconnected);
                let client = Client::with_sender(out, move || {
                    disconnected.lock().unwrap().push(client_id);
                }).unwrap();

                connected.lock().unwrap().push((client_id, client.clone()));
                client
            }).unwrap();
        });
    }

    let mut game_server = GameServer::new().unwrap();
    let mut clients: HashMap<ClientId, Client> = HashMap::new();
    'main: loop {
        {
            let mut connected = connected.lock().unwrap();
            for (id, client) in connected.drain(0..) {
                clients.insert(id, client.clone());
                game_server.add_player(id).unwrap();
                println!("Client {} connected!", id);
            }
        }
        {
            let mut disconnected = disconnected.lock().unwrap();
            for id in disconnected.drain(0..) {
                clients.remove(&id);
                game_server.remove_player(id).unwrap();
                println!("Client {} disconnected!", id);
            }
        }

        for (id, client) in clients.iter_mut() {
            let mut incoming = client.incoming().unwrap();
            for msg in incoming.drain(0..) {
                let packets: Vec<Packet> = deserialize(&msg).unwrap();
                game_server.handle_incoming_packets(id, &packets).unwrap();
            }
        }

        game_server.update(0.016).unwrap();

        for (id, client) in clients.iter_mut() {
            let packets = game_server.take_outgoing_packets(id).unwrap();
            if packets.len() > 0 {
                let msg = serialize(&packets).unwrap();
                client.send(msg).unwrap();
            }
        }

        thread::sleep(Duration::from_millis(16));
    }
}
