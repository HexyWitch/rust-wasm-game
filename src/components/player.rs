use specs::{Component, HashMapStorage};

pub struct Player;

impl Component for Player {
    type Storage = HashMapStorage<Self>;
}
