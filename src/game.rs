use std::rc::Rc;
use std::f32;

use png;

use platform::input::{InputEvent, Key};
use assets::Image;
use math::Vec2;

use texture_image::TextureImage;
use render_interface::RenderInterface;

const SHIP_IMAGE: &[u8] = include_bytes!("../assets/ship.png");

struct PlayerInput {
    left: bool,
    right: bool,
    thrust: bool,
}

pub struct Game {
    input: PlayerInput,
    ship_texture: TextureImage,
    ship_position: Vec2,
    ship_angle: f32,
}

impl Game {
    pub fn new() -> Game {
        let decoder = png::Decoder::new(SHIP_IMAGE);
        let (info, mut reader) = decoder.read_info().unwrap();
        let mut buf = vec![0; info.buffer_size()];
        reader.next_frame(&mut buf).unwrap();
        let ship_image = Rc::new(Image {
            data: buf,
            width: info.width,
            height: info.height,
        });

        Game {
            input: PlayerInput {
                left: false,
                right: false,
                thrust: false,
            },
            ship_texture: TextureImage::new(ship_image),
            ship_position: Vec2(320.0, 240.0),
            ship_angle: 0.0,
        }
    }

    pub fn update(&mut self, dt: f64, input_events: &[InputEvent]) {
        for e in input_events {
            match *e {
                InputEvent::KeyDown(Key::A) => {
                    self.input.left = true;
                }
                InputEvent::KeyUp(Key::A) => {
                    self.input.left = false;
                }
                InputEvent::KeyDown(Key::D) => {
                    self.input.right = true;
                }
                InputEvent::KeyUp(Key::D) => {
                    self.input.right = false;
                }
                InputEvent::KeyDown(Key::W) => {
                    self.input.thrust = true;
                }
                InputEvent::KeyUp(Key::W) => {
                    self.input.thrust = false;
                }
                _ => {}
            }
        }

        let rotate_speed = 5.0;
        if self.input.left {
            self.ship_angle += rotate_speed * dt as f32;
        }
        if self.input.right {
            self.ship_angle -= rotate_speed * dt as f32;
        }

        if self.input.thrust {
            let velocity = Vec2::with_angle(self.ship_angle) * 150.0;
            self.ship_position += velocity * dt as f32;
        }
    }

    pub fn render(&self, renderer: &mut RenderInterface) {
        renderer
            .draw_texture(&self.ship_texture, self.ship_position, 4.0, self.ship_angle)
            .unwrap();
    }
}
