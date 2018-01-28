#![feature(set_stdio)]
#![feature(conservative_impl_trait)]

#[cfg(target_arch = "wasm32")]
mod platform_js;

#[cfg(target_arch = "wasm32")]
use std::io;

#[cfg(target_arch = "wasm32")]
use platform_js::websocket::WebSocket;
#[cfg(target_arch = "wasm32")]
use platform_js::console_writer::ConsoleWriter;
#[cfg(target_arch = "wasm32")]
pub use platform_js::exports::*;

#[cfg(target_arch = "wasm32")]
fn init() -> Box<FnMut()> {
    let mut socket = WebSocket::connect("ws://localhost:3012").unwrap();
    let mut ping = true;
    let mut x = 1;
    Box::new(move || {
        if socket.open() {
            if ping {
                let msg = format!("{}", x);
                println!("Ping: {}", x);
                socket.send(&msg).unwrap();
                ping = false;
                x += 1;
            }

            while let Some(msg) = socket.next() {
                println!("Pong: {}", msg);
                ping = true;
            }
        }
    })
}

#[cfg(target_arch = "wasm32")]
fn main() {
    io::set_print(Some(Box::new(ConsoleWriter::new())));
    io::set_panic(Some(Box::new(ConsoleWriter::new())));

    let mut update = init();
    platform_js::js::set_main_loop_callback(|| update());
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // let mut update = init();
    // loop {
    //     update();
    //     thread::sleep(std::time::Duration::from_millis(16));
    // }
}
