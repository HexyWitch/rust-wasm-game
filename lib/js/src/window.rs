use wasm_bindgen::prelude::*;

use webgl;

#[wasm_bindgen(module = "./window")]
extern "C" {
    pub type Window;

    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str, input_handler: InputHandler) -> Window;

    #[wasm_bindgen(method)]
    pub fn set_main_loop(this: &Window, cb: MainLoopCallback);

    #[wasm_bindgen(method)]
    pub fn gl_context(this: &Window) -> webgl::GlContext;

    #[wasm_bindgen(static = Window)]
    pub fn log(msg: &str);
}

#[wasm_bindgen]
pub struct MainLoopCallback(pub Box<FnMut() + 'static>);

#[wasm_bindgen]
impl MainLoopCallback {
    pub fn call(&mut self) {
        (*self.0)();
    }
}

type MouseX = i32;
type MouseY = i32;
type MouseButton = i8;
type Key = i32;

type MouseMoveCallback = Box<FnMut(MouseX, MouseY) + 'static>;
type MouseButtonCallback = Box<FnMut(MouseButton, MouseX, MouseY) + 'static>;
type KeyboardCallback = Box<FnMut(Key) + 'static>;

#[wasm_bindgen]
pub struct InputHandler {
    mouse_move: Option<MouseMoveCallback>,
    mouse_down: Option<MouseButtonCallback>,
    mouse_up: Option<MouseButtonCallback>,
    key_down: Option<KeyboardCallback>,
    key_up: Option<KeyboardCallback>,
}

#[wasm_bindgen]
impl InputHandler {
    pub fn mouse_move(&mut self, x: MouseX, y: MouseY) {
        if let Some(ref mut mouse_move) = self.mouse_move {
            (*mouse_move)(x, y);
        }
    }
    pub fn mouse_down(&mut self, button: MouseButton, x: MouseX, y: MouseY) {
        if let Some(ref mut mouse_down) = self.mouse_down {
            (*mouse_down)(button, x, y);
        }
    }
    pub fn mouse_up(&mut self, button: MouseButton, x: MouseX, y: MouseY) {
        if let Some(ref mut mouse_up) = self.mouse_up {
            (*mouse_up)(button, x, y);
        }
    }
    pub fn key_down(&mut self, key: Key) {
        if let Some(ref mut key_down) = self.key_down {
            (*key_down)(key);
        }
    }
    pub fn key_up(&mut self, key: Key) {
        if let Some(ref mut key_up) = self.key_up {
            (*key_up)(key);
        }
    }
}

impl InputHandler {
    pub fn new() -> InputHandler {
        InputHandler {
            mouse_move: None,
            mouse_down: None,
            mouse_up: None,
            key_down: None,
            key_up: None,
        }
    }

    pub fn set_mouse_move<T: FnMut(MouseX, MouseY) + 'static>(&mut self, f: T) {
        self.mouse_move = Some(Box::new(f));
    }
    pub fn set_mouse_down<T: FnMut(MouseButton, MouseX, MouseY) + 'static>(&mut self, f: T) {
        self.mouse_down = Some(Box::new(f));
    }
    pub fn set_mouse_up<T: FnMut(MouseButton, MouseX, MouseY) + 'static>(&mut self, f: T) {
        self.mouse_up = Some(Box::new(f));
    }
    pub fn set_key_down<T: FnMut(Key) + 'static>(&mut self, f: T) {
        self.key_down = Some(Box::new(f));
    }
    pub fn set_key_up<T: FnMut(Key) + 'static>(&mut self, f: T) {
        self.key_up = Some(Box::new(f));
    }
}
