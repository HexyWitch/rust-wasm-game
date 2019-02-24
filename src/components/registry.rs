use embla::ecs::World;

pub use super::player::*;
pub use super::prefab::*;
pub use super::sprite::*;
pub use super::transform::*;
pub use super::velocity::*;

pub fn register_components(world: &mut World) {
    world.register_component::<Player>();
    world.register_component::<Sprite>();
    world.register_component::<Transform>();
    world.register_component::<Velocity>();
    world.register_component::<Prefab>();
}
