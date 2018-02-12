extern crate assets;
extern crate gl;
extern crate platform;
extern crate sdl2;

pub mod renderer_gl;

use std::thread;
use std::time::Duration;
use sdl2::video::GLProfile;

use platform::Application;

pub fn run<T: Application + 'static>() {
    let sdl_context = sdl2::init().unwrap();
    let sdl_video = sdl_context.video().unwrap();
    {
        let gl_attr = sdl_video.gl_attr();
        gl_attr.set_context_major_version(2);
        gl_attr.set_context_minor_version(0);
        gl_attr.set_context_profile(GLProfile::GLES);
        gl_attr.set_double_buffer(true);
        gl_attr.set_depth_size(24);
    }

    let window = sdl_video
        .window("Thing", 640, 480)
        .opengl()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    gl::load_with(|name| sdl_video.gl_get_proc_address(name) as *const _);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut application = T::new();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                _ => {}
            }
        }

        application.update(0.016);

        window.gl_swap_window();
        thread::sleep(Duration::from_millis(16));
    }
}
