use specs::{Component, VecStorage};

use embla::graphics::TextureImage;

pub struct Sprite {
    pub texture: TextureImage,
}

impl Component for Sprite {
    type Storage = VecStorage<Self>;
}
