use failure::Error;

use texture_image::TextureImage;
use math::Vec2;

pub trait RenderInterface {
    fn draw_texture(
        &mut self,
        texture: &TextureImage,
        position: Vec2,
        scale: f32,
        rotation: f32,
    ) -> Result<(), Error>;
}
