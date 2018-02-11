use std::f32;
use std::rc::Rc;

use simple_renderer::SimpleRenderer;
use rendering::TextureImage;
use renderer_gl::GLRenderer;
use core::Image;
use vec2::Vec2;

pub fn init() -> Box<FnMut()> {
    println!("Start the application!");

    let mut renderer = SimpleRenderer::<GLRenderer>::new((640.0, 480.0)).unwrap();

    #[cfg_attr(rustfmt, rustfmt_skip)]
    let example_image = Rc::new(Image {
        data: vec![255,  0,  0,255,   0,255,  0,255,
                     0,  0,255,255, 255,255,255,255],
        width: 2,
        height: 2,
    });

    let example_texture = TextureImage::new(example_image);
    let mut timer = 0.0;
    let position = Vec2(200.0, 200.0);
    Box::new(move || {
        timer = timer + 0.016;

        let angle = (timer % 1.0) * f32::consts::PI * 2.0;
        //let offset = Vec2(angle.cos(), angle.sin()) * 20.0;
        renderer
            .draw_texture(&example_texture, position, 100.0, 0.0)
            .unwrap();

        renderer.do_render().unwrap();
    })
}
