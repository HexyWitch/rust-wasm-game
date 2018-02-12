use std::rc::Rc;
use std::f32;

use assets::Image;
use math::Vec2;

use texture_image::TextureImage;
use render_interface::RenderInterface;

pub struct Game {
    box_texture: TextureImage,
    box_position: Vec2,
    box_angle: f32,
    timer: f32,
}

impl Game {
    pub fn new() -> Game {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let example_image = Rc::new(Image {
            data: vec![255,  0,  0,255,   0,255,  0,255,
                        0,  0,255,255, 255,255,255,255],
            width: 2,
            height: 2,
        });

        Game {
            box_texture: TextureImage::new(example_image),
            box_position: Vec2(320.0, 240.0),
            box_angle: 0.0,
            timer: 0.0,
        }
    }

    pub fn update(&mut self, dt: f64) {
        let cycle_time = 2.5;
        self.timer += dt as f32;
        while self.timer > cycle_time {
            self.timer -= cycle_time;
        }

        self.box_angle = (self.timer / cycle_time) * f32::consts::PI * 2.0;
        let offset = Vec2(self.box_angle.cos(), self.box_angle.sin()) * 150.0;
        self.box_position = Vec2(320.0, 240.0) + offset;
    }

    pub fn render(&self, renderer: &mut RenderInterface) {
        renderer
            .draw_texture(&self.box_texture, self.box_position, 100.0, self.box_angle)
            .unwrap();
    }
}
