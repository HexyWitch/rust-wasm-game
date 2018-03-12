#![feature(proc_macro)]
extern crate game;
extern crate js;
extern crate platform_web;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

use game::ClientServerApplication;

use platform_web::WebPlatformApi;

pub fn main() {
    platform_web::run::<ClientServerApplication<WebPlatformApi>>();
}

#[wasm_bindgen]
pub fn web_main() {
    main();
}
