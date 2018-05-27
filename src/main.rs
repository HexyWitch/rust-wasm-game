extern crate embla;
extern crate failure;

mod client_server_application;
mod game_client;
mod render_interface;
mod renderer;

pub use client_server_application::ClientServerApplication;

pub fn main() {
    embla::run(|| {
        let mut application = ClientServerApplication::new().unwrap();
        move |dt, input| {
            application.update(dt, input)?;

            Ok(())
        }
    });
}
