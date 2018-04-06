#![feature(proc_macro, wasm_import_module, wasm_custom_section)]
#![allow(non_camel_case_types)]

extern crate wasm_bindgen;

pub mod webgl;
pub mod websocket;
pub mod window;

use wasm_bindgen::prelude::*;

pub fn bootstrap() {
    webgl::bootstrap();
    websocket::bootstrap();
    window::bootstrap();
}

#[wasm_bindgen]
extern "C" {
    pub type console;

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    pub fn eval(s: &str);
}
