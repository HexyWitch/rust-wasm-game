extern crate game;
extern crate js;
extern crate platform_web;

pub use js::exports::*;

use game::application::GameApplication;
use platform_web::renderer_webgl::WebGLRenderer;

fn main() {
    platform_web::run::<GameApplication<WebGLRenderer>>();
}
