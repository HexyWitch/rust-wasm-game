#![feature(set_stdio)]
extern crate assets;
extern crate js;
extern crate platform;

mod console_writer;
pub mod renderer_webgl;

use std::io;

use platform::Application;

use self::console_writer::ConsoleWriter;

pub fn run<T: Application + 'static>() {
    io::set_print(Some(Box::new(ConsoleWriter::new())));
    io::set_panic(Some(Box::new(ConsoleWriter::new())));

    let mut application = T::new();
    js::set_main_loop_callback(move || {
        application.update(0.016);
    });
}
