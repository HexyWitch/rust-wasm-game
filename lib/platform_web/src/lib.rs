#![feature(set_stdio)]
extern crate core;
#[macro_use]
extern crate failure;
extern crate js;
extern crate platform;

use std::io;

mod input;
mod console_writer;
pub mod websocket;
pub mod renderer_webgl;

use platform::{Application, PlatformApi};
use platform::input::{Input, InputEvent};
use input::to_input_event;

use self::console_writer::ConsoleWriter;

pub struct WebPlatformApi();

impl PlatformApi for WebPlatformApi {
    type Renderer = renderer_webgl::WebGLRenderer;
    type Socket = websocket::JsWebSocket;
}

pub fn run<T: Application + 'static>() {
    io::set_print(Some(Box::new(ConsoleWriter::new())));
    io::set_panic(Some(Box::new(ConsoleWriter::new())));

    js::webgl::gl_load_context("window");
    let mut application = T::new().unwrap();
    let mut input = Input::new();
    js::set_main_loop_callback(move || {
        // let input_events: Vec<InputEvent> = input_events.iter().map(to_input_event).collect();
        // input.update(&input_events);

        application.update(0.016, &input).unwrap();
    });
}
