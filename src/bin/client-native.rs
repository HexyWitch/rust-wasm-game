extern crate game;
extern crate platform_native;

use game::ClientServerApplication;

use platform_native::NativePlatformApi;

fn main() {
    platform_native::run::<ClientServerApplication<NativePlatformApi>>();
}
