use failure::Error;

use platform::input::{Input, Key};

use core::math::Vec2;

use render_interface::RenderInterface;
use game_client::Assets;

pub struct Ship {
    position: Vec2,
    angle: f32,
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            position: Vec2::zero(),
            angle: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32, input: &Input) -> Result<(), Error> {
        let rotate_speed = 5.0;
        if input.key_is_down(&Key::A) {
            self.angle += rotate_speed * dt as f32;
        }
        if input.key_is_down(&Key::D) {
            self.angle -= rotate_speed * dt as f32;
        }

        if input.key_is_down(&Key::W) {
            let velocity = Vec2::with_angle(self.angle) * 150.0;
            self.position += velocity * dt as f32;
        }
        Ok(())
    }

    pub fn render(&self, assets: &Assets, renderer: &mut RenderInterface) -> Result<(), Error> {
        renderer.draw_texture(&assets.ship, self.position, 1.0, self.angle)?;
        Ok(())
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position
    }
}
