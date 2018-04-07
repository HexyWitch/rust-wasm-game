use std::borrow::Borrow;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::rc::Rc;

use embla::assets::Image;

#[derive(Clone)]
pub struct TextureImage {
    id: u64,
    width: u32,
    height: u32,
    image: Rc<Image>,
}

impl TextureImage {
    pub fn new(image: Rc<Image>) -> TextureImage {
        let mut hasher: DefaultHasher = DefaultHasher::new();
        hasher.write(&image.data);
        hasher.write_u32(image.width);
        hasher.write_u32(image.height);
        TextureImage {
            id: hasher.finish(),
            width: image.width,
            height: image.height,
            image: image,
        }
    }
    pub fn image(&self) -> &Image {
        self.image.borrow()
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn id(&self) -> u64 {
        self.id
    }
}
