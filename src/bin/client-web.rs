extern crate game;

extern crate js;
extern crate platform_web;

pub use js::exports::*;

use game::ClientServerApplication;

use platform_web::WebPlatformApi;

fn main() {
    platform_web::run::<ClientServerApplication<WebPlatformApi>>();
}
