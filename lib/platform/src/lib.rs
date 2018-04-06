#![feature(set_stdio)]

extern crate core;
#[macro_use]
extern crate failure;

#[cfg(target_arch = "wasm32")]
extern crate js;

#[cfg(not(target_arch = "wasm32"))]
extern crate gl;
#[cfg(not(target_arch = "wasm32"))]
extern crate sdl2;
#[cfg(not(target_arch = "wasm32"))]
extern crate ws;

#[cfg(not(target_arch = "wasm32"))]
pub mod native;
#[cfg(not(target_arch = "wasm32"))]
pub use native as platform;

#[cfg(target_arch = "wasm32")]
pub mod web;
#[cfg(target_arch = "wasm32")]
pub use web as platform;

pub mod input;
pub mod rendering_api;

pub use platform::*;
