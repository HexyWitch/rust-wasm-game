use core::math::Vec2;

pub type ClientId = u32;

pub enum Packet {
    ClientConnected,
    ClientInit { player_position: Vec2 },
}
