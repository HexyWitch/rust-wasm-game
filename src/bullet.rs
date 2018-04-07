use embla::input::Input;
use embla::math::Vec2;
use failure::Error;

use game_client::Assets;
use net::BulletNetUpdate;
use render_interface::RenderInterface;

pub struct Bullet {
    age: f32,
    position: Vec2,
    velocity: Vec2,
}

impl Bullet {
    pub fn new(position: Vec2, velocity: Vec2) -> Bullet {
        Bullet {
            age: 0.0,
            position,
            velocity,
        }
    }

    pub fn position(&self) -> Vec2 {
        self.position.clone()
    }

    pub fn velocity(&self) -> Vec2 {
        self.velocity.clone()
    }

    pub fn client_update(&mut self, _dt: f32, _input: &Input) -> Result<(), Error> {
        Ok(())
    }

    pub fn server_update(&mut self, dt: f32) -> Result<(), Error> {
        self.age += dt;
        self.position += self.velocity * dt;
        Ok(())
    }

    pub fn render(&self, assets: &Assets, renderer: &mut RenderInterface) -> Result<(), Error> {
        renderer.draw_texture(&assets.bullet, self.position, 1.0, self.velocity.angle())?;
        Ok(())
    }

    pub fn dead(&self) -> bool {
        self.age >= 1.0
    }

    pub fn get_net_update(&self) -> BulletNetUpdate {
        BulletNetUpdate {
            position: self.position,
        }
    }

    pub fn set_net_update(&mut self, update: &BulletNetUpdate) {
        self.position = update.position;
    }
}
