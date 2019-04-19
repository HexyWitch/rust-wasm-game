use specs::{Component, DenseVecStorage};

use embla::math::Vec2;

pub struct Velocity(pub Vec2<f32>);

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}
