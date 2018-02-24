#![feature(conservative_impl_trait)]
#[macro_use]
extern crate failure;
extern crate platform;
extern crate png;

extern crate core;

mod texture_atlas;
mod texture_image;
mod render_interface;
mod ship;
mod game_client;
mod game_server;
mod renderer;
mod net;
mod client_server_application;

pub use client_server_application::ClientServerApplication;
