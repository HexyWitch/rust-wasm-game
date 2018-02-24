use std::collections::HashMap;
use std::f32;
use std::mem;

use failure::Error;

use core::math::Vec2;

use ship::Ship;
use net::{ClientId, Packet};

pub struct GameServer {
    players: HashMap<ClientId, Ship>,
    outgoing_packets: HashMap<ClientId, Vec<Packet>>
}

impl GameServer {
    pub fn new() -> Result<GameServer, Error> {
        Ok(GameServer {
            players: HashMap::new(),
            outgoing_packets: HashMap::new(),
        })
    }

    pub fn add_player(&mut self, id: ClientId) -> Result<(), Error> {
        let mut ship = Ship::new();
        let position = Vec2::new(300.0, 300.0);
        ship.set_position(position);
        self.players.insert(id, ship);

        self.outgoing_packets.insert(id, Vec::new());
        self.outgoing_packets.get_mut(&id).unwrap().push(Packet::ClientInit{player_position: position});
        Ok(())
    }

    pub fn update(&mut self, _dt: f32) -> Result<(), Error> {
        Ok(())
    }

    pub fn handle_incoming_packets(&mut self, _packets: &[Packet]) -> Result<(), Error> {
        Ok(())
    }

    pub fn take_outgoing_packets(&mut self, client_id: &ClientId) -> Result<Vec<Packet>, Error> {
        let packets = self.outgoing_packets.get_mut(client_id).ok_or_else(|| format_err!("could not get outgoing packets for non-existent client {}", client_id))?;
        let packets = mem::replace(packets, Vec::new());
        Ok(packets)
    }
}
