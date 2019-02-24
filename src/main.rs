extern crate bincode;
extern crate embla;
#[macro_use]
extern crate failure;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod client_server_application;
mod components;
mod game_client;
mod game_server;
mod net;
mod prefab;
mod render_interface;
mod renderer;

use embla::math::Vec2;
use embla::window::WindowSettings;

pub use client_server_application::ClientServerApplication;

pub fn main() {
    embla::init(|mut context| {
        let window = context
            .window(
                WindowSettings::new()
                    .title("Space shooty".to_string())
                    .size(Vec2::new(640, 480))
                    .canvas_id("window".to_string()),
            )
            .unwrap();

        let mut application = ClientServerApplication::new(window).unwrap();
        move |dt, input| {
            application.update(dt, input)?;

            Ok(())
        }
    });
}
