use std::f32;
use std::rc::Rc;
use std::io;

use simple_renderer::SimpleRenderer;
use rendering::TextureImage;
use renderer_webgl::GLRenderer;
use core::Image;
use vec2::Vec2;
use platform_web::console_writer::ConsoleWriter;

pub fn init() -> Box<FnMut()> {
    io::set_print(Some(Box::new(ConsoleWriter::new())));
    io::set_panic(Some(Box::new(ConsoleWriter::new())));

    println!("Start the application!");
    let mut renderer =
        SimpleRenderer::<GLRenderer>::new((640.0, 480.0)).expect("Error creating renderer");

    #[cfg_attr(rustfmt, rustfmt_skip)]
    let example_image = Rc::new(Image {
        data: vec![255,  0,  0,255,   0,255,  0,255,
                     0,  0,255,255, 255,255,255,255],
        width: 2,
        height: 2,
    });

    let example_texture = TextureImage::new(example_image);
    let cycle_time = 2.5;
    let mut timer = 0.0;
    let position = Vec2(320.0, 240.0);
    Box::new(move || {
        timer = timer + 0.016;
        while timer > cycle_time {
            timer -= cycle_time;
        }

        let angle = (timer / cycle_time) * f32::consts::PI * 2.0;
        let offset = Vec2(angle.cos(), angle.sin()) * 150.0;
        renderer
            .draw_texture(&example_texture, position + offset, 100.0, angle)
            .unwrap();

        renderer.do_render().unwrap();
    })
}
