use failure::Error;

use embla::math::Vec2;

use embla::graphics::TextureImage;

pub trait RenderInterface {
    fn draw_texture(
        &mut self,
        texture: &TextureImage,
        position: Vec2,
        scale: f32,
        rotation: f32,
    ) -> Result<(), Error>;
}
