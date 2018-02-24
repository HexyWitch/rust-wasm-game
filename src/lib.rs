#![feature(conservative_impl_trait)]
extern crate bincode;
#[macro_use]
extern crate failure;
extern crate platform;
extern crate png;
extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate core;

mod texture_atlas;
mod texture_image;
mod render_interface;
mod ship;
mod game_client;
mod game_server;
mod renderer;
pub mod net;
mod client_application;
mod client_server_application;

pub use client_application::ClientApplication;
pub use client_server_application::ClientServerApplication;
pub use game_server::GameServer;
