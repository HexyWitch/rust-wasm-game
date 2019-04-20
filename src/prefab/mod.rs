mod player;
mod registry;

use failure::Error;
use specs::{Entity, World};

pub use self::player::*;
pub use self::registry::{PrefabIndex, Registry};

pub trait Prefab {
    fn create(world: &mut World) -> Result<Entity, Error>;
}

pub fn register_prefabs(registry: &mut Registry) {
    registry.register_prefab::<PlayerPrefab>();
}
