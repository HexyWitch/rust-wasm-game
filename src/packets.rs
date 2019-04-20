use std::collections::HashMap;

use net::{ComponentDelta, ComponentStore, EntityId};
use prefab::PrefabIndex;

#[derive(Serialize, Deserialize)]
pub struct EntityStore {
    pub entity_id: EntityId,
    pub prefab: PrefabIndex,
    pub components: Vec<ComponentStore>,
}

#[derive(Serialize, Deserialize)]
pub enum Packet {
    Connect,
    Initialize,
    CreateEntity(EntityStore),
    Update(HashMap<EntityId, Vec<ComponentDelta>>),
    PlayerInput { left: bool, right: bool, up: bool },
}
