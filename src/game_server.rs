use std::collections::HashMap;
use std::f32;
use std::mem;

use failure::Error;

use embla::math::Vec2;

use bullet::Bullet;
use net::{BulletNetState, ClientId, Packet};
use ship::Ship;

pub struct PlayerInput {
    left: bool,
    right: bool,
    forward: bool,
    shoot: bool,
}

struct PlayerData {
    input: PlayerInput,
    ship_id: i32,
}

pub struct GameServer {
    players: HashMap<ClientId, PlayerData>,
    last_id: i32,
    ships: HashMap<i32, Ship>,
    last_bullet_id: i32,
    bullets: HashMap<i32, Bullet>,
    outgoing_packets: HashMap<ClientId, Vec<Packet>>,
}

impl GameServer {
    pub fn new() -> Result<GameServer, Error> {
        Ok(GameServer {
            players: HashMap::new(),
            last_id: 0,
            ships: HashMap::new(),
            last_bullet_id: 0,
            bullets: HashMap::new(),
            outgoing_packets: HashMap::new(),
        })
    }

    pub fn add_player(&mut self, id: ClientId) -> Result<(), Error> {
        let mut ship = Ship::new();
        let position = Vec2::new(300.0, 300.0);
        ship.set_position(position);

        let ship_id = self.last_id;
        self.last_id += 1;

        self.ships.insert(ship_id, ship);
        self.players.insert(
            id,
            PlayerData {
                input: PlayerInput {
                    left: false,
                    right: false,
                    forward: false,
                    shoot: false,
                },
                ship_id: ship_id,
            },
        );

        self.outgoing_packets.insert(id, Vec::new());
        self.outgoing_packets
            .get_mut(&id)
            .unwrap()
            .push(Packet::ClientInit {
                ship_data: self.ships
                    .iter()
                    .map(|(k, s)| (*k, s.get_net_update()))
                    .collect(),
                bullet_data: self.bullets
                    .iter()
                    .map(|(id, bullet)| {
                        (
                            *id,
                            BulletNetState {
                                position: bullet.position(),
                                velocity: bullet.velocity(),
                            },
                        )
                    })
                    .collect(),
            });

        for (_, outgoing) in self.outgoing_packets.iter_mut() {
            outgoing.push(Packet::CreateShip(ship_id));
        }
        Ok(())
    }

    pub fn remove_player(&mut self, id: ClientId) -> Result<(), Error> {
        let player_data = self.players.remove(&id).unwrap();
        self.ships.remove(&player_data.ship_id);

        for (_, outgoing) in self.outgoing_packets.iter_mut() {
            outgoing.push(Packet::DestroyShip(player_data.ship_id));
        }
        Ok(())
    }

    pub fn spawn_bullet(&mut self, position: Vec2, velocity: Vec2) -> Result<(), Error> {
        let bullet = Bullet::new(position, velocity);
        self.bullets.insert(self.last_bullet_id, bullet);

        for (_, outgoing) in self.outgoing_packets.iter_mut() {
            outgoing.push(Packet::SpawnBullet {
                id: self.last_bullet_id,
                position: position,
                velocity: velocity,
            });
        }

        self.last_bullet_id += 1;
        Ok(())
    }

    pub fn update(&mut self, dt: f32) -> Result<(), Error> {
        let mut spawn_bullets = Vec::new();
        for (_client_id, player_data) in self.players.iter_mut() {
            let ship = self.ships.get_mut(&player_data.ship_id).unwrap();
            ship.set_controls(
                player_data.input.left,
                player_data.input.right,
                player_data.input.forward,
            );
            ship.server_update(dt)?;

            if player_data.input.shoot {
                spawn_bullets.push((ship.position(), ship.angle()));
                player_data.input.shoot = false;
            }
        }
        for (position, angle) in spawn_bullets {
            self.spawn_bullet(position, Vec2::with_angle(angle) * 400.0)?;
        }

        for (id, ship) in self.ships.iter() {
            let update_packet = Packet::ShipUpdate {
                id: *id,
                update: ship.get_net_update(),
            };
            for (_, outgoing) in self.outgoing_packets.iter_mut() {
                outgoing.push(update_packet.clone());
            }
        }

        let mut dead_bullets = Vec::new();
        for (id, bullet) in self.bullets.iter_mut() {
            bullet.server_update(dt)?;
            for (_, outgoing) in self.outgoing_packets.iter_mut() {
                outgoing.push(Packet::BulletUpdate {
                    id: *id,
                    update: bullet.get_net_update(),
                });
            }
            if bullet.dead() {
                dead_bullets.push(*id);
            }
        }
        for id in dead_bullets {
            self.bullets.remove(&id);

            for (_, outgoing) in self.outgoing_packets.iter_mut() {
                outgoing.push(Packet::DestroyBullet(id));
            }
        }

        Ok(())
    }

    pub fn handle_incoming_packets(
        &mut self,
        client_id: &ClientId,
        packets: &[Packet],
    ) -> Result<(), Error> {
        let player_data = self.players
            .get_mut(client_id)
            .ok_or_else(|| format_err!("received packet for non existent client"))?;
        for p in packets {
            match *p {
                Packet::PlayerInput {
                    left,
                    right,
                    forward,
                    shoot,
                } => {
                    player_data.input.left = left;
                    player_data.input.right = right;
                    player_data.input.forward = forward;
                    player_data.input.shoot = shoot;
                }
                _ => {}
            }
        }
        Ok(())
    }

    pub fn take_outgoing_packets(&mut self, client_id: &ClientId) -> Result<Vec<Packet>, Error> {
        let packets = self.outgoing_packets.get_mut(client_id).ok_or_else(|| {
            format_err!(
                "could not get outgoing packets for non-existent client {}",
                client_id
            )
        })?;
        let packets = mem::replace(packets, Vec::new());
        Ok(packets)
    }
}
