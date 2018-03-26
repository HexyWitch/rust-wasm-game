#![feature(set_stdio)]
extern crate core;
#[macro_use]
extern crate failure;
extern crate js;
extern crate platform;

use std::io;

mod console_writer;
mod input;
pub mod renderer_webgl;
pub mod websocket;
mod window;

use js::webgl;
use platform::input::Input;
use platform::{Application, PlatformApi};
use window::Window;

use self::console_writer::ConsoleWriter;

pub struct WebPlatformApi();

impl PlatformApi for WebPlatformApi {
    type Renderer = renderer_webgl::WebGLRenderer;
    type Socket = websocket::Websocket;
}

pub fn run<T: Application + 'static>() {
    js::bootstrap();

    io::set_print(Some(Box::new(ConsoleWriter::new())));
    io::set_panic(Some(Box::new(ConsoleWriter::new())));

    let canvas_id = "window";
    let mut window = Window::new(canvas_id).unwrap();
    webgl::set_global_context(webgl::get_canvas_context(canvas_id));

    let mut event_dispatch = window.events();
    let mut application = T::new().unwrap();
    let mut input = Input::new();
    window.set_main_loop(move || {
        let events = event_dispatch.input_events();
        input.update(&events);

        application.update(0.016, &input).unwrap();
    });
}
