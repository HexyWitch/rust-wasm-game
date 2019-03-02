use failure::Error;
use std::mem;

use embla::math::Vec2;
use embla_ecs::World;

use components;
use components::{Player, Transform};
use net;
use net::Packet;
use prefab;
use prefab::{PlayerConfig, PlayerPrefab};

pub struct GameServer {
    world: World,
    outgoing: Vec<Packet>,
    prefabs: prefab::Registry,
}

impl GameServer {
    pub fn new() -> Result<GameServer, Error> {
        let mut world = World::new();
        components::register_components(&mut world);

        let mut prefabs = prefab::Registry::new();
        prefab::register_prefabs(&mut prefabs);

        Ok(GameServer {
            world,
            outgoing: Vec::new(),
            prefabs,
        })
    }

    pub fn handle_incoming(&mut self, packet: &Packet) -> Result<(), Error> {
        match *packet {
            Packet::Connect => {
                self.prefabs.create::<PlayerPrefab>(
                    &mut self.world,
                    PlayerConfig {
                        position: Vec2::new(200.0, 200.0),
                    },
                )?;

                let world_state = self.world_state()?;
                self.outgoing.push(Packet::WorldState(world_state));
            }
            _ => {
                return Err(format_err!("server received unexpected packet"));
            }
        }

        Ok(())
    }

    pub fn take_outgoing(&mut self) -> Vec<Packet> {
        mem::replace(&mut self.outgoing, Vec::new())
    }

    pub fn update(&mut self, _dt: f64) -> Result<(), Error> {
        Ok(())
    }

    fn world_state(&mut self) -> Result<Vec<net::EntityStore>, Error> {
        let mut players = Vec::new();
        for (mut transform, _) in self.world.iter::<(Transform, Player)>() {
            players.push(self.prefabs.serialize::<PlayerPrefab>(&PlayerConfig {
                position: transform.position,
            })?);
        }

        Ok(players)
    }
}
