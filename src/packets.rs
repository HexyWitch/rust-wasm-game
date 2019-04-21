use net::{ComponentDelta, ComponentStore, EntityId};
use prefab::PrefabIndex;

#[derive(Serialize, Deserialize)]
pub struct EntitiesStore {
    pub entities: Vec<(EntityId, PrefabIndex)>,
    pub components: ComponentStore,
}

#[derive(Serialize, Deserialize)]
pub enum Packet {
    Connect,
    Initialize,
    CreateEntities(EntitiesStore),
    Update(ComponentDelta),
    PlayerInput { left: bool, right: bool, up: bool },
}
