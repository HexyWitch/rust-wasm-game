#[cfg(target_arch = "wasm32")]
pub use platform_js::websocket;

#[cfg(not(target_arch = "wasm32"))]
pub use platform_native::websocket;
