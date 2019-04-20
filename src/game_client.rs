use failure::Error;
use std::collections::HashMap;
use std::mem;

use embla::input::{Input, Key};
use specs::{Join, World};

use components;
use components::{Networked, Sprite, Transform};
use net::{ComponentStore, EntityId, NetComponentAdapter};
use packets::{EntityStore, Packet};
use prefab;
use render_interface::RenderInterface;

#[derive(Clone, Copy, PartialEq, Eq)]
enum GameState {
    Start,
    Connecting,
    Running,
}

pub struct GameClient {
    world: World,
    prefabs: prefab::Registry,
    outgoing: Vec<Packet>,
    state: GameState,

    net_adapter: NetComponentAdapter,
    net_load: HashMap<EntityId, Vec<ComponentStore>>,
}

impl GameClient {
    pub fn new() -> Result<GameClient, Error> {
        let mut net_adapter = NetComponentAdapter::new();
        let mut world = World::new();
        components::register_components(&mut world, &mut net_adapter);

        let mut prefabs = prefab::Registry::new();
        prefab::register_prefabs(&mut prefabs);

        Ok(GameClient {
            world,
            prefabs,
            outgoing: Vec::new(),
            state: GameState::Start,

            net_adapter,
            net_load: HashMap::new(),
        })
    }

    pub fn handle_incoming(&mut self, packet: Packet) -> Result<(), Error> {
        match packet {
            Packet::Initialize => {
                if self.state == GameState::Connecting {
                    self.state = GameState::Running;
                } else {
                    return Err(format_err!("unexpected initialize packet"));
                }
            }
            Packet::CreateEntity(EntityStore {
                entity_id,
                prefab,
                components,
            }) => {
                let e = self.prefabs.instantiate(&mut self.world, prefab)?;
                self.world
                    .write_storage::<Networked>()
                    .insert(e, Networked { entity_id, prefab })?;

                self.net_load.insert(entity_id, components);
            }
            Packet::Update(net_deltas) => {
                self.net_adapter
                    .write_delta::<Transform>(&self.world, &net_deltas);
            }
            _ => {
                return Err(format_err!("client received unexpected packet"));
            }
        }

        Ok(())
    }

    pub fn take_outgoing(&mut self) -> Vec<Packet> {
        mem::replace(&mut self.outgoing, Vec::new())
    }

    pub fn update(&mut self, _: f64, input: &Input) -> Result<(), Error> {
        let net_load = mem::replace(&mut self.net_load, HashMap::new());
        self.net_adapter
            .net_load::<Transform>(&self.world, &net_load);

        match self.state {
            GameState::Start => {
                self.outgoing.push(Packet::Connect);
                self.state = GameState::Connecting;
            }
            GameState::Connecting => {}
            GameState::Running => {
                self.outgoing.push(Packet::PlayerInput {
                    left: input.key_is_down(&Key::A),
                    right: input.key_is_down(&Key::D),
                    up: input.key_is_down(&Key::W),
                });
            }
        }

        Ok(())
    }

    pub fn render(&mut self, renderer: &mut RenderInterface) -> Result<(), Error> {
        let transform = self.world.read_storage::<Transform>();
        let sprite = self.world.read_storage::<Sprite>();
        for (transform, sprite) in (&transform, &sprite).join() {
            renderer.draw_texture(
                &sprite.texture,
                transform.position,
                transform.scale,
                transform.rotation,
            )?;
        }

        Ok(())
    }
}
