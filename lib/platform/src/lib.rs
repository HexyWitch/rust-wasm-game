extern crate assets;
extern crate failure;

pub mod rendering_api;
pub mod input;

use failure::Error;

pub trait PlatformApi {
    type Renderer: rendering_api::Renderer;
}

use input::InputEvent;
pub trait Application {
    fn new() -> Result<Self, Error>
    where
        Self: Sized;
    fn update(&mut self, dt: f64, input_events: &[InputEvent]) -> Result<(), Error>;
}
