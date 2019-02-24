use embla::math::Vec2;

#[derive(Clone)]
pub struct Transform {
    pub position: Vec2<f32>,
    pub scale: f32,
    pub rotation: f32,
}
