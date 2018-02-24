extern crate game;
extern crate platform_native;

use game::ClientApplication;

use platform_native::NativePlatformApi;

fn main() {
    platform_native::run::<ClientApplication<NativePlatformApi>>();
}
