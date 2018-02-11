#![feature(set_stdio)]
#![feature(conservative_impl_trait)]

mod core;
mod rendering;
mod renderer_webgl;
mod vec2;
mod simple_renderer;

#[cfg(not(target_arch = "wasm32"))]
mod platform_native;
#[cfg(target_arch = "wasm32")]
mod platform_web;

#[cfg(target_arch = "wasm32")]
mod application_web;

#[cfg(target_arch = "wasm32")]
pub use platform_web::exports::*;

#[cfg(target_arch = "wasm32")]
fn main() {
    let mut update = application_web::init();
    unsafe {
        platform_web::js::set_main_loop_callback(move || update());
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Does nothing
}
