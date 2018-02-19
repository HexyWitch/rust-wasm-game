extern crate game;

#[cfg(not(target_arch = "wasm32"))]
extern crate platform_native;

#[cfg(target_arch = "wasm32")]
extern crate js;
#[cfg(target_arch = "wasm32")]
extern crate platform_web;

#[cfg(target_arch = "wasm32")]
pub use js::exports::*;

use game::application::GameApplication;

#[cfg(not(target_arch = "wasm32"))]
use platform_native::NativePlatformApi;
#[cfg(target_arch = "wasm32")]
use platform_web::WebPlatformApi;

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    platform_native::run::<GameApplication<NativePlatformApi>>();
    #[cfg(target_arch = "wasm32")]
    platform_web::run::<GameApplication<WebPlatformApi>>();
}
