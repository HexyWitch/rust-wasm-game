use failure::Error;
use std::f32;
use std::rc::Rc;

use embla::assets::image_from_png;
use embla::ecs::World;
use embla::graphics::TextureImage;
use embla::input::{Input, Key};
use embla::math::Vec2;

use render_interface::RenderInterface;

#[derive(Clone)]
pub struct Transform {
    position: Vec2,
    scale: f32,
    rotation: f32,
}

pub struct Velocity(Vec2);

pub struct Sprite {
    texture: TextureImage,
}

pub struct GameClient {
    world: World,
}

pub struct Player;

impl GameClient {
    pub fn new() -> Result<GameClient, Error> {
        let mut world = World::new();
        world.register_component::<Transform>();
        world.register_component::<Velocity>();
        world.register_component::<Player>();
        world.register_component::<Sprite>();

        world
            .add_entity()
            .insert(Transform {
                position: Vec2::new(200.0, 300.0),
                scale: 1.0,
                rotation: 0.0,
            })?
            .insert(Velocity(Vec2::new(0.0, 0.0)))?
            .insert(Sprite {
                texture: TextureImage::new(Rc::new(image_from_png(include_bytes!(
                    "../assets/ship.png"
                ))?)),
            })?
            .insert(Player)?
            .id();

        Ok(GameClient { world })
    }

    pub fn update(&mut self, dt: f32, input: &Input) -> Result<(), Error> {
        for (mut transform, mut velocity, _) in self.world
            .with_components::<(Transform, Velocity, Player)>()
        {
            if input.key_is_down(&Key::A) {
                transform.rotation += 5.0 * dt;
            }
            if input.key_is_down(&Key::D) {
                transform.rotation -= 5.0 * dt;
            }
            if input.key_is_down(&Key::W) {
                velocity.0 = Vec2::with_angle(transform.rotation) * 300.0;
            } else {
                velocity.0 = Vec2::zero();
            }
        }

        for (mut transform, velocity) in self.world.with_components::<(Transform, Velocity)>() {
            transform.position += velocity.0 * dt;
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
