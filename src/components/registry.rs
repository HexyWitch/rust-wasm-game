use specs::World;

use net::NetComponentAdapter;

pub use super::networked::*;
pub use super::player::*;
pub use super::sprite::*;
pub use super::transform::*;
pub use super::velocity::*;

pub fn register_components(world: &mut World, net_adapter: &mut NetComponentAdapter) {
    world.register::<Player>();
    world.register::<Sprite>();
    world.register::<Transform>();
    world.register::<Velocity>();
    world.register::<Networked>();

    net_adapter.register_component::<Transform>();
}
