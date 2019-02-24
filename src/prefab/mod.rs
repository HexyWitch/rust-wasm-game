mod player;
mod registry;

pub use self::player::*;

pub use self::registry::{Prefab, PrefabId, Registry};

pub fn register_prefabs(registry: &mut Registry) {
    registry.register_prefab::<PlayerPrefab>();
}
