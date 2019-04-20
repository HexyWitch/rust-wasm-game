use bincode;
use embla::math::Vec2;
use specs::{Component, VecStorage};

use net::NetComponent;

#[derive(Clone, Serialize, Deserialize)]
pub struct Transform {
    pub position: Vec2<f32>,
    pub scale: f32,
    pub rotation: f32,
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            position: Vec2::zero(),
            scale: 1.0,
            rotation: 0.0,
        }
    }
}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}

impl NetComponent for Transform {
    fn net_store(&self) -> Vec<u8> {
        bincode::serialize(self).expect("error serializing Transform")
    }
    fn net_load(&mut self, data: &[u8]) {
        *self = bincode::deserialize(data).expect("error deserializing Transform");
    }

    fn read_delta(&self) -> Vec<u8> {
        bincode::serialize(self).expect("error serializing Transform")
    }
    fn write_delta(&mut self, data: &[u8]) {
        *self = bincode::deserialize(data).expect("error deserializing Transform");
    }
}
