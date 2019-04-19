use specs::{Component, DenseVecStorage};

use prefab::PrefabId;

pub struct Prefab(pub PrefabId);

impl Component for Prefab {
    type Storage = DenseVecStorage<Self>;
}
