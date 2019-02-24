use failure::Error;
use std::f32;
use std::mem;

use embla::ecs::World;
use embla::input::{Input, Key};
use embla::math::Vec2;

use components;
use components::{Player, Sprite, Transform, Velocity};
use net::Packet;
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
}

impl GameClient {
    pub fn new() -> Result<GameClient, Error> {
        let mut world = World::new();
        components::register_components(&mut world);

        let mut prefabs = prefab::Registry::new();
        prefab::register_prefabs(&mut prefabs);

        Ok(GameClient {
            world,
            prefabs,
            outgoing: Vec::new(),
            state: GameState::Start,
        })
    }

    pub fn handle_incoming(&mut self, packet: &Packet) -> Result<(), Error> {
        match *packet {
            Packet::WorldState(ref entity_stores) => {
                if self.state == GameState::Connecting {
                    self.state = GameState::Running;
                } else {
                    return Err(format_err!("unexpected world state packet"));
                }
                for store in entity_stores.iter() {
                    self.prefabs.load(&mut self.world, store)?;
                }
            }
            Packet::CreateEntity(ref store) => {
                self.prefabs.load(&mut self.world, store)?;
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

    pub fn update(&mut self, dt: f64, input: &Input) -> Result<(), Error> {
        match self.state {
            GameState::Start => {
                self.outgoing.push(Packet::Connect);
                self.state = GameState::Connecting;
            }
            GameState::Connecting => {}
            GameState::Running => {
                for (mut transform, mut velocity, _) in self
                    .world
                    .with_components::<(Transform, Velocity, Player)>()
                {
                    if input.key_is_down(&Key::A) {
                        transform.rotation += 5.0 * dt as f32;
                    }
                    if input.key_is_down(&Key::D) {
                        transform.rotation -= 5.0 * dt as f32;
                    }
                    if input.key_is_down(&Key::W) {
                        velocity.0 = Vec2::with_angle(transform.rotation) * 300.0;
                    } else {
                        velocity.0 = Vec2::zero();
                    }
                }

                for (mut transform, velocity) in
                    self.world.with_components::<(Transform, Velocity)>()
                {
                    transform.position += velocity.0 * dt as f32;
                }
            }
        }

        Ok(())
    }

    pub fn render(&mut self, renderer: &mut RenderInterface) -> Result<(), Error> {
        for (sprite, transform) in self.world.with_components::<(Sprite, Transform)>() {
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
