#![feature(proc_macro)]
#![allow(non_camel_case_types)]

extern crate failure;
extern crate wasm_bindgen;

pub mod webgl;
pub mod websocket;
pub mod window;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type console;

    #[wasm_bindgen(static = console)]
    pub fn log(s: &str);
}
