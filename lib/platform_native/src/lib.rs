extern crate core;
#[macro_use]
extern crate failure;
extern crate gl;
extern crate platform;
extern crate sdl2;

mod input;
pub mod websocket;
pub mod renderer_gl;

use std::thread;
use std::time::Duration;
use sdl2::video::GLProfile;
use sdl2::event::Event;

use platform::{Application, PlatformApi};
use platform::input::{Input, InputEvent};

use input::{to_key, to_mouse_button};

pub struct NativePlatformApi();

impl PlatformApi for NativePlatformApi {
    type Renderer = renderer_gl::GLRenderer;
    type Socket = websocket::NativeWebSocket;
}

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
    let mut application = T::new().unwrap();
    let mut input = Input::new();
    'main: loop {
        let mut input_events = Vec::new();
        for event in event_pump.poll_iter() {
            match event {
                Event::MouseMotion { x, y, .. } => {
                    input_events.push(InputEvent::MouseMove(x, y));
                }
                Event::MouseButtonDown {
                    mouse_btn, x, y, ..
                } => {
                    input_events.push(InputEvent::MouseDown {
                        button: to_mouse_button(mouse_btn),
                        position: (x, y),
                    });
                }
                Event::MouseButtonUp {
                    mouse_btn, x, y, ..
                } => {
                    input_events.push(InputEvent::MouseUp {
                        button: to_mouse_button(mouse_btn),
                        position: (x, y),
                    });
                }
                Event::KeyDown {
                    keycode: Some(key), ..
                } => input_events.push(InputEvent::KeyDown(to_key(key))),
                Event::KeyUp {
                    keycode: Some(key), ..
                } => input_events.push(InputEvent::KeyUp(to_key(key))),
                _ => {}
            }
        }
        input.update(&input_events);

        application.update(0.016, &input).unwrap();

        window.gl_swap_window();
        thread::sleep(Duration::from_millis(16));
    }
}
