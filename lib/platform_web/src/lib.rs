#![feature(set_stdio)]
extern crate assets;
extern crate js;
extern crate platform;

mod input;
mod console_writer;
pub mod renderer_webgl;

use std::io;

use platform::Application;
use platform::input::InputEvent;
use input::to_input_event;

use self::console_writer::ConsoleWriter;

pub fn run<T: Application + 'static>() {
    io::set_print(Some(Box::new(ConsoleWriter::new())));
    io::set_panic(Some(Box::new(ConsoleWriter::new())));

    let mut application = T::new();
    js::set_main_loop_callback(move |input_events| {
        let input_events: Vec<InputEvent> = input_events.iter().map(to_input_event).collect();

        application.update(0.016, &input_events);
    });
}
