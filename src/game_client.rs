use failure::Error;
use std::f32;
use std::mem;

use embla::graphics::TextureImage;
use embla::input::{Input, Key};
use embla::math::Vec2;
use specs::{Join, ReadStorage, RunNow, System, World, WriteStorage};

use components;
use components::{Sprite, Transform, Velocity};
use net::Packet;
use prefab;
use render_interface::RenderInterface;

static TIMESTEP: f64 = 1.0 / 60.0;

#[derive(Clone, Copy, PartialEq, Eq)]
enum GameState {
    Start,
    Connecting,
    Running,
}

struct InputSystem {
    input: Input,
}

impl InputSystem {
    fn new() -> InputSystem {
        InputSystem {
            input: Input::new(),
        }
    }

    fn set_input(&mut self, input: Input) {
        self.input = input;
    }
}

impl<'a> System<'a> for InputSystem {
    type SystemData = (WriteStorage<'a, Transform>, WriteStorage<'a, Velocity>);
    fn run(&mut self, (mut transform, mut vel): Self::SystemData) {
        for (transform, velocity) in (&mut transform, &mut vel).join() {
            if self.input.key_is_down(&Key::A) {
                transform.rotation += 5.0 * TIMESTEP as f32;
            }
            if self.input.key_is_down(&Key::D) {
                transform.rotation -= 5.0 * TIMESTEP as f32;
            }
            if self.input.key_is_down(&Key::W) {
                velocity.0 = Vec2::with_angle(transform.rotation) * 300.0;
            } else {
                velocity.0 = Vec2::zero();
            }
        }
    }
}

struct MovementSystem {}

impl MovementSystem {
    pub fn new() -> MovementSystem {
        MovementSystem {}
    }
}

impl<'a> System<'a> for MovementSystem {
    type SystemData = (WriteStorage<'a, Transform>, ReadStorage<'a, Velocity>);
    fn run(&mut self, (mut transform, vel): Self::SystemData) {
        for (transform, velocity) in (&mut transform, &vel).join() {
            transform.position += velocity.0 * TIMESTEP as f32;
        }
    }
}

pub struct Drawable {
    pub texture: TextureImage,
    pub position: Vec2<f32>,
    pub scale: f32,
    pub rotation: f32,
}

pub struct RenderSystem {
    drawables: Vec<Drawable>,
}

impl RenderSystem {
    pub fn new() -> RenderSystem {
        RenderSystem {
            drawables: Vec::new(),
        }
    }

    pub fn take_drawables(&mut self) -> Vec<Drawable> {
        mem::replace(&mut self.drawables, Vec::new())
    }
}

impl<'a> System<'a> for RenderSystem {
    type SystemData = (ReadStorage<'a, Transform>, ReadStorage<'a, Sprite>);
    fn run(&mut self, (transform, sprite): Self::SystemData) {
        for (transform, sprite) in (&transform, &sprite).join() {
            self.drawables.push(Drawable {
                texture: sprite.texture.clone(),
                position: transform.position,
                scale: transform.scale,
                rotation: transform.rotation,
            })
        }
    }
}

pub struct ClientSystems {
    input: InputSystem,
    movement: MovementSystem,
    render: RenderSystem,
}

impl ClientSystems {
    pub fn new() -> ClientSystems {
        ClientSystems {
            input: InputSystem::new(),
            movement: MovementSystem::new(),
            render: RenderSystem::new(),
        }
    }
}

pub struct GameClient {
    world: World,
    prefabs: prefab::Registry,
    outgoing: Vec<Packet>,
    state: GameState,
    systems: ClientSystems,
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
            systems: ClientSystems::new(),
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

    pub fn update(&mut self, _: f64, input: &Input) -> Result<(), Error> {
        self.systems.input.set_input(input.clone());

        match self.state {
            GameState::Start => {
                self.outgoing.push(Packet::Connect);
                self.state = GameState::Connecting;
            }
            GameState::Connecting => {}
            GameState::Running => {
                self.systems.input.run_now(&self.world.res);
                self.systems.movement.run_now(&self.world.res);
            }
        }

        Ok(())
    }

    pub fn render(&mut self, renderer: &mut RenderInterface) -> Result<(), Error> {
        self.systems.render.run_now(&self.world.res);
        for drawable in self.systems.render.take_drawables() {
            renderer.draw_texture(
                &drawable.texture,
                drawable.position,
                drawable.scale,
                drawable.rotation,
            )?;
        }

        Ok(())
    }
}
