use specs::{Component, DenseVecStorage};

use net::EntityId;
use prefab::PrefabIndex;

pub struct Networked {
    pub entity_id: EntityId,
    pub prefab: PrefabIndex,
}

impl Component for Networked {
    type Storage = DenseVecStorage<Self>;
}
