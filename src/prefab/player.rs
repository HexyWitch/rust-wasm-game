use std::f32;
use std::sync::Arc;

use failure::Error;
use specs::{Builder, Entity, World};

use embla::assets::image_from_png;
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

    fn store(world: &mut World, e: Entity) -> Result<Self::Config, Error> {
        let position = world
            .read_storage::<Transform>()
            .get(e)
            .ok_or_else(|| format_err!("entity not found"))?
            .position;

        Ok(PlayerConfig { position })
    }
    fn create(world: &mut World, config: Self::Config) -> Result<Entity, Error> {
        Ok(world
            .create_entity()
            .with(Transform {
                position: config.position,
                scale: 1.0,
                rotation: 0.0,
            })
            .with(Velocity(Vec2::new(0.0, 0.0)))
            .with(Sprite {
                texture: TextureImage::new(Arc::new(image_from_png(include_bytes!(
                    "../../assets/ship.png"
                ))?)),
            })
            .with(Player)
            .build())
    }
}
