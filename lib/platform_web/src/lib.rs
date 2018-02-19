extern crate assets;
#[macro_use]
extern crate failure;
extern crate js;
extern crate platform;

#[cfg(feature = "set_stdio")]
use std::io;

mod input;
#[cfg(feature = "set_stdio")]
mod console_writer;
pub mod renderer_webgl;

use platform::{Application, PlatformApi};
use platform::input::InputEvent;
use input::to_input_event;

#[cfg(feature = "set_stdio")]
use self::console_writer::ConsoleWriter;

pub struct WebPlatformApi();

impl PlatformApi for WebPlatformApi {
    type Renderer = renderer_webgl::WebGLRenderer;
}

pub fn run<T: Application + 'static>() {
    #[cfg(feature = "set_stdio")]
    io::set_print(Some(Box::new(ConsoleWriter::new())));
    #[cfg(feature = "set_stdio")]
    io::set_panic(Some(Box::new(ConsoleWriter::new())));

    let mut application = T::new().unwrap();
    js::set_main_loop_callback(move |input_events| {
        let input_events: Vec<InputEvent> = input_events.iter().map(to_input_event).collect();

        application.update(0.016, &input_events).unwrap();
    });
}
