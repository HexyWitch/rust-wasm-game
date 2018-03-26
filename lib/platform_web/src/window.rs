use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

use failure::Error;

use js;
use js::webgl;
use js::window::{CanvasWindow, InputHandler as JsInputHandler, MainLoopCallback};
use platform::input::InputEvent;

use input::{to_key, to_mouse_button};

type InputEvents = Rc<RefCell<Vec<InputEvent>>>;

pub struct Window {
    js_window: CanvasWindow,
    input_events: InputEvents,
}

fn input_handler(input_events: &Rc<RefCell<Vec<InputEvent>>>) -> JsInputHandler {
    let mut handler = JsInputHandler::new();

    let events = Rc::clone(input_events);
    handler.set_mouse_move(move |x, y| {
        events.borrow_mut().push(InputEvent::MouseMove(x, y));
    });

    let events = Rc::clone(input_events);
    handler.set_mouse_down(move |button, x, y| {
        events.borrow_mut().push(InputEvent::MouseDown {
            button: to_mouse_button(button),
            position: (x, y),
        });
    });

    let events = Rc::clone(input_events);
    handler.set_mouse_up(move |button, x, y| {
        events.borrow_mut().push(InputEvent::MouseUp {
            button: to_mouse_button(button),
            position: (x, y),
        });
    });

    let events = Rc::clone(input_events);
    handler.set_key_down(move |key| {
        events.borrow_mut().push(InputEvent::KeyDown(to_key(key)));
    });

    let events = Rc::clone(input_events);
    handler.set_key_up(move |key| {
        events.borrow_mut().push(InputEvent::KeyUp(to_key(key)));
    });

    handler
}

impl Window {
    pub fn new(canvas_id: &str) -> Result<Window, Error> {
        let input_events = Rc::new(RefCell::new(Vec::new()));
        let handler = input_handler(&input_events);
        Ok(Window {
            js_window: js::window::create_canvas_window(canvas_id, handler),
            input_events,
        })
    }

    pub fn events(&mut self) -> EventDispatch {
        EventDispatch(self.input_events.clone())
    }

    pub fn set_main_loop<T: FnMut() + 'static>(self, f: T) {
        js::window::set_main_loop(MainLoopCallback(Box::new(f)));
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        //js::window::delete_canvas_window(&self.js_window);
    }
}

pub struct EventDispatch(InputEvents);

impl EventDispatch {
    pub fn input_events(&mut self) -> Vec<InputEvent> {
        let mut input_events = self.0.borrow_mut();
        mem::replace(&mut *input_events, Vec::new())
    }
}
