extern crate game;
extern crate platform_native;

use game::application::GameApplication;
use platform_native::renderer_gl::GLRenderer;

fn main() {
    platform_native::run::<GameApplication<GLRenderer>>();
}
