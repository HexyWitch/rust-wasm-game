use specs::{Component, HashMapStorage};

pub struct Player {
    pub up: bool,
    pub left: bool,
    pub right: bool,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            up: false,
            left: false,
            right: false,
        }
    }
}

impl Component for Player {
    type Storage = HashMapStorage<Self>;
}
