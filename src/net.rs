use std::collections::HashMap;

use embla::math::Vec2;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShipNetUpdate {
    pub position: Vec2,
    pub angle: f32,
}

pub type ClientId = u32;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Packet {
    // From client to server
    PlayerInput {
        left: bool,
        right: bool,
        forward: bool,
    },

    // From server to client
    ClientInit {
        ship_data: HashMap<i32, ShipNetUpdate>,
    },
    CreateShip(i32),
    DestroyShip(i32),
    ShipUpdate {
        id: i32,
        update: ShipNetUpdate,
    },
}
