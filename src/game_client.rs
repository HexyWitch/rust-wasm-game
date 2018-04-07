use failure::Error;
use std::collections::HashMap;
use std::f32;
use std::mem;
use std::rc::Rc;

use embla::assets::image_from_png;
use embla::graphics::TextureImage;
use embla::input::{Input, Key};

use bullet::Bullet;
use net::{BulletNetState, Packet};
use render_interface::RenderInterface;
use ship::Ship;

pub struct Assets {
    pub ship: TextureImage,
    pub bullet: TextureImage,
}

enum GameState {
    Connecting,
    Running {
        ships: HashMap<i32, Ship>,
        bullets: HashMap<i32, Bullet>,
    },
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
            bullet: TextureImage::new(Rc::new(image_from_png(include_bytes!(
                "../assets/bullet.png"
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
            GameState::Running {
                ref mut ships,
                ref mut bullets,
            } => {
                self.outgoing_packets.push(Packet::PlayerInput {
                    left: input.key_is_down(&Key::A),
                    right: input.key_is_down(&Key::D),
                    forward: input.key_is_down(&Key::W),
                    shoot: input.key_is_pressed(&Key::Space),
                });

                for (_, ship) in ships.iter_mut() {
                    ship.client_update(dt, input)?;
                }

                for (_, bullet) in bullets.iter_mut() {
                    bullet.client_update(dt, input)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn render(&self, renderer: &mut RenderInterface) -> Result<(), Error> {
        match self.state {
            GameState::Running {
                ref ships,
                ref bullets,
            } => {
                for (_, ship) in ships.iter() {
                    ship.render(&self.assets, renderer)?;
                }
                for (_, bullet) in bullets.iter() {
                    bullet.render(&self.assets, renderer)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn handle_incoming_packets(&mut self, packets: &[Packet]) -> Result<(), Error> {
        for p in packets {
            match self.state {
                GameState::Connecting => match *p {
                    Packet::ClientInit {
                        ref ship_data,
                        ref bullet_data,
                    } => {
                        let mut ships = HashMap::new();
                        for (id, net_data) in ship_data.iter() {
                            let mut ship = Ship::new();
                            ship.set_net_update(net_data);
                            ships.insert(*id, ship);
                        }
                        let mut bullets = bullet_data
                            .iter()
                            .map(|(id, BulletNetState { position, velocity })| {
                                (*id, Bullet::new(position.clone(), velocity.clone()))
                            })
                            .collect();

                        self.state = GameState::Running { ships, bullets };
                    }
                    _ => {}
                },
                GameState::Running {
                    ref mut ships,
                    ref mut bullets,
                } => match *p {
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
                    Packet::SpawnBullet {
                        id,
                        position,
                        velocity,
                    } => {
                        bullets.insert(id, Bullet::new(position, velocity));
                    }
                    Packet::DestroyBullet(id) => {
                        bullets.remove(&id);
                    }
                    Packet::BulletUpdate { id, ref update } => {
                        let bullet = bullets
                            .get_mut(&id)
                            .ok_or_else(|| format_err!("could not update bullet with id {}", id))?;
                        bullet.set_net_update(update);
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
