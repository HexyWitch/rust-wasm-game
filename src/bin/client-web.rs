extern crate game;

extern crate js;
extern crate platform_web;

pub use js::exports::*;

use game::ClientApplication;

use platform_web::WebPlatformApi;

fn main() {
    platform_web::run::<ClientApplication<WebPlatformApi>>();
}
