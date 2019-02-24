use std::f32;
use std::rc::Rc;

use failure::Error;

use embla::assets::image_from_png;
use embla::ecs::World;
use embla::graphics::TextureImage;
use embla::math::Vec2;

use components::{Player, Sprite, Transform, Velocity};
use prefab::Prefab;

pub enum PlayerPrefab {}

#[derive(Serialize, Deserialize)]
pub struct PlayerConfig {
    pub position: Vec2<f32>,
}

impl Prefab for PlayerPrefab {
    type Config = PlayerConfig;

    fn store(world: &mut World, e: usize) -> Result<Self::Config, Error> {
        let position = world
            .get_component::<Transform>(e)
            .ok_or_else(|| format_err!("invalid entity"))?
            .position;

        Ok(PlayerConfig { position })
    }
    fn create(world: &mut World, config: Self::Config) -> Result<usize, Error> {
        Ok(world
            .add_entity()
            .insert(Transform {
                position: config.position,
                scale: 1.0,
                rotation: 0.0,
            })?
            .insert(Velocity(Vec2::new(0.0, 0.0)))?
            .insert(Sprite {
                texture: TextureImage::new(Rc::new(image_from_png(include_bytes!(
                    "../../assets/ship.png"
                ))?)),
            })?
            .insert(Player)?
            .id())
    }
}
