extern crate assets;

pub mod rendering_api;
pub mod input;

pub trait PlatformApi {
    type Renderer: rendering_api::Renderer;
}

use input::InputEvent;
pub trait Application {
    fn new() -> Self;
    fn update(&mut self, dt: f64, input_events: &[InputEvent]);
}
