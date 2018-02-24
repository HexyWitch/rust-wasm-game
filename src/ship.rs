use failure::Error;

use platform::input::Input;

use core::math::Vec2;

use render_interface::RenderInterface;
use game_client::Assets;
use net::ShipNetUpdate;

pub struct Ship {
    position: Vec2,
    angle: f32,
    left: bool,
    right: bool,
    forward: bool,
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            position: Vec2::zero(),
            angle: 0.0,
            left: false,
            right: false,
            forward: false,
        }
    }

    pub fn client_update(&mut self, _dt: f32, _input: &Input) -> Result<(), Error> {
        Ok(())
    }

    pub fn server_update(&mut self, dt: f32) -> Result<(), Error> {
        let rotate_speed = 5.0;
        if self.left {
            self.angle += rotate_speed * dt as f32;
        }
        if self.right {
            self.angle -= rotate_speed * dt as f32;
        }

        if self.forward {
            let velocity = Vec2::with_angle(self.angle) * 150.0;
            self.position += velocity * dt as f32;
        }
        Ok(())
    }

    pub fn render(&self, assets: &Assets, renderer: &mut RenderInterface) -> Result<(), Error> {
        renderer.draw_texture(&assets.ship, self.position, 1.0, self.angle)?;
        Ok(())
    }

    pub fn set_controls(&mut self, left: bool, right: bool, forward: bool) {
        self.left = left;
        self.right = right;
        self.forward = forward;
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position
    }

    pub fn get_net_update(&self) -> ShipNetUpdate {
        ShipNetUpdate {
            position: self.position,
            angle: self.angle,
        }
    }

    pub fn set_net_update(&mut self, update: &ShipNetUpdate) {
        self.position = update.position;
        self.angle = update.angle;
    }
}
