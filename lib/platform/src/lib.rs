extern crate core;
extern crate failure;

pub mod websocket;
pub mod rendering_api;
pub mod input;

use failure::Error;

pub trait PlatformApi {
    type Renderer: rendering_api::Renderer;
    type Socket: websocket::WebSocket;
}

use input::Input;
pub trait Application {
    fn new() -> Result<Self, Error>
    where
        Self: Sized;
    fn update(&mut self, dt: f32, input: &Input) -> Result<(), Error>;
}
