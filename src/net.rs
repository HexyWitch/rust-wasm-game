use std::collections::HashMap;

use embla::math::Vec2;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShipNetUpdate {
    pub position: Vec2,
    pub angle: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BulletNetState {
    pub position: Vec2,
    pub velocity: Vec2,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BulletNetUpdate {
    pub position: Vec2,
}

pub type ClientId = u32;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Packet {
    // From client to server
    PlayerInput {
        left: bool,
        right: bool,
        forward: bool,
        shoot: bool,
    },

    // From server to client
    ClientInit {
        ship_data: HashMap<i32, ShipNetUpdate>,
        bullet_data: HashMap<i32, BulletNetState>,
    },
    CreateShip(i32),
    DestroyShip(i32),
    ShipUpdate {
        id: i32,
        update: ShipNetUpdate,
    },
    SpawnBullet {
        id: i32,
        position: Vec2,
        velocity: Vec2,
    },
    DestroyBullet(i32),
    BulletUpdate {
        id: i32,
        update: BulletNetUpdate,
    },
}
