extern crate game;
extern crate js;
extern crate platform_web;

use game::ClientApplication;

use platform_web::WebPlatformApi;

pub fn main() {
    platform_web::run::<ClientApplication<WebPlatformApi>>();
}
