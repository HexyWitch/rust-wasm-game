extern crate bincode;
extern crate failure;
extern crate game;
extern crate platform;
extern crate platform_native;
extern crate ws;

use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use bincode::{deserialize, serialize};

use platform::websocket::WebSocket;
use platform_native::websocket::Client;

use game::GameServer;
use game::net::{ClientId, Packet};

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
