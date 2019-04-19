use specs::World;

pub use super::player::*;
pub use super::prefab::*;
pub use super::sprite::*;
pub use super::transform::*;
pub use super::velocity::*;

pub fn register_components(world: &mut World) {
    world.register::<Player>();
    world.register::<Sprite>();
    world.register::<Transform>();
    world.register::<Velocity>();
    world.register::<Prefab>();
}
