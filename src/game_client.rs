use std::rc::Rc;
use std::f32;
use std::collections::HashMap;
use std::mem;
use failure::Error;

use core::assets::image_from_png;
use platform::input::{Input, Key};

use texture_image::TextureImage;
use render_interface::RenderInterface;
use ship::Ship;
use net::Packet;

pub struct Assets {
    pub ship: TextureImage,
}

enum GameState {
    Connecting,
    Running { ships: HashMap<i32, Ship> },
}

pub struct GameClient {
    assets: Assets,
    state: GameState,
    outgoing_packets: Vec<Packet>,
}

impl GameClient {
    pub fn new() -> Result<GameClient, Error> {
        let assets = Assets {
            ship: TextureImage::new(Rc::new(image_from_png(include_bytes!(
                "../assets/ship.png"
            ))?)),
        };

        Ok(GameClient {
            assets: assets,
            state: GameState::Connecting,
            outgoing_packets: Vec::new(),
        })
    }

    pub fn update(&mut self, dt: f32, input: &Input) -> Result<(), Error> {
        match self.state {
            GameState::Running { ref mut ships } => {
                let left = input.key_is_down(&Key::A);
                let right = input.key_is_down(&Key::D);
                let forward = input.key_is_down(&Key::W);
                self.outgoing_packets.push(Packet::PlayerInput {
                    left,
                    right,
                    forward,
                });

                for (_, ship) in ships.iter_mut() {
                    ship.client_update(dt, input)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn render(&self, renderer: &mut RenderInterface) -> Result<(), Error> {
        match self.state {
            GameState::Running { ref ships } => for (_, ship) in ships.iter() {
                ship.render(&self.assets, renderer)?;
            },
            _ => {}
        }
        Ok(())
    }

    pub fn handle_incoming_packets(&mut self, packets: &[Packet]) -> Result<(), Error> {
        for p in packets {
            match self.state {
                GameState::Connecting => match *p {
                    Packet::ClientInit { ref ship_data } => {
                        println!("Initialize game!");

                        let mut ships = HashMap::new();
                        for (id, net_data) in ship_data.iter() {
                            let mut ship = Ship::new();
                            ship.set_net_update(net_data);
                            ships.insert(*id, ship);
                        }
                        self.state = GameState::Running { ships: ships };
                    }
                    _ => {}
                },
                GameState::Running { ref mut ships } => match *p {
                    Packet::CreateShip(id) => {
                        let new_ship = Ship::new();
                        ships.insert(id, new_ship);
                    }
                    Packet::DestroyShip(id) => {
                        ships.remove(&id);
                    }
                    Packet::ShipUpdate { id, ref update } => {
                        let ship = ships
                            .get_mut(&id)
                            .ok_or_else(|| format_err!("could not update ship with id {}", id))?;
                        ship.set_net_update(update);
                    }
                    _ => {}
                },
            }
        }
        Ok(())
    }

    pub fn take_outgoing_packets(&mut self) -> Result<Vec<Packet>, Error> {
        let packets = mem::replace(&mut self.outgoing_packets, Vec::new());
        Ok(packets)
    }
}
