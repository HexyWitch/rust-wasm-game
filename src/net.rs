pub type EntityStore = Vec<u8>;

#[derive(Serialize, Deserialize)]
pub enum Packet {
    Connect,
    WorldState(Vec<EntityStore>),
    CreateEntity(EntityStore),
}
