extern crate game;
extern crate platform_native;

use game::application::GameApplication;
use platform_native::NativePlatformApi;

fn main() {
    platform_native::run::<GameApplication<NativePlatformApi>>();
}
