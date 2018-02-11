#![feature(set_stdio)]
#![feature(conservative_impl_trait)]

mod platform;
#[cfg(target_arch = "wasm32")]
mod platform_js;
#[cfg(not(target_arch = "wasm32"))]
mod platform_native;

mod application;
mod core;
mod rendering;
mod renderer_gl;
mod vec2;
mod simple_renderer;

#[cfg(target_arch = "wasm32")]
use std::io;
#[cfg(not(target_arch = "wasm32"))]
use std::thread;

#[cfg(target_arch = "wasm32")]
pub use platform_js::exports::*;
#[cfg(target_arch = "wasm32")]
use platform_js::console_writer::ConsoleWriter;

#[cfg(target_arch = "wasm32")]
fn main() {
    io::set_print(Some(Box::new(ConsoleWriter::new())));
    io::set_panic(Some(Box::new(ConsoleWriter::new())));

    let mut update = application::init();
    unsafe {
        platform_js::js::set_main_loop_callback(move || update());
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let mut update = application::init();
    loop {
        update();
        thread::sleep(std::time::Duration::from_millis(16));
    }
}
