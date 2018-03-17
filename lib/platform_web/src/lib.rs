#![feature(set_stdio)]
extern crate core;
#[macro_use]
extern crate failure;
extern crate js;
extern crate platform;

use std::io;

mod input;
mod console_writer;
mod window;
pub mod websocket;
pub mod renderer_webgl;

use platform::{Application, PlatformApi};
use platform::input::Input;
use window::Window;
use js::webgl;

use self::console_writer::ConsoleWriter;

pub struct WebPlatformApi();

impl PlatformApi for WebPlatformApi {
    type Renderer = renderer_webgl::WebGLRenderer;
    type Socket = websocket::Websocket;
}

pub fn run<T: Application + 'static>() {
    io::set_print(Some(Box::new(ConsoleWriter::new())));
    io::set_panic(Some(Box::new(ConsoleWriter::new())));

    let mut window = Window::new("window").unwrap();
    webgl::set_context(window.gl_context());

    let mut event_dispatch = window.events();
    let mut application = T::new().unwrap();
    let mut input = Input::new();
    window.set_main_loop(move || {
        let events = event_dispatch.input_events();
        input.update(&events);

        application.update(0.016, &input).unwrap();
    });
}
