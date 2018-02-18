pub mod webgl;
pub mod websocket;
pub mod exports;

use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr::null_mut;
use std::cell::RefCell;
use std::mem;

type MainLoopCallback = unsafe extern "C" fn(*const c_void, usize);

extern "C" {
    fn js_console_log(ptr: *const u8);
    fn js_set_main_loop(fn_ptr: MainLoopCallback);
}

thread_local!(static MAIN_LOOP_CALLBACK: RefCell<*mut c_void> = RefCell::new(null_mut()));

#[allow(dead_code)]
pub fn console_log(s: &str) {
    let c_str = CString::new(s).unwrap();
    unsafe {
        js_console_log(c_str.as_ptr() as *const u8);
    }
}

// Will exit the application and pass callback into javascript, where it will be called on a timer
// Should only be called once at the exit point of your application
pub fn set_main_loop_callback<T>(callback: T)
where
    T: FnMut(&[InputEvent]) + 'static,
{
    MAIN_LOOP_CALLBACK
        .with(|cb| *cb.borrow_mut() = Box::into_raw(Box::new(callback)) as *mut c_void);

    pub unsafe extern "C" fn wrapper<T>(events_ptr: *const c_void, size: usize)
    where
        T: FnMut(&[InputEvent]),
    {
        let input_events = read_input_events(events_ptr, size);
        MAIN_LOOP_CALLBACK.with(|cb| {
            let callback = *cb.borrow_mut() as *mut T;
            (*callback)(&input_events)
        })
    }

    unsafe {
        js_set_main_loop(wrapper::<T>);
    }
}

#[derive(Debug)]
pub enum InputEvent {
    MouseMove(i32, i32),
    MouseDown { button: i8, position: (i32, i32) },
    MouseUp { button: i8, position: (i32, i32) },
    KeyDown(i32),
    KeyUp(i32),
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum InputEventType {
    MouseMove,
    MouseDown,
    MouseUp,
    KeyDown,
    KeyUp,
}

// Raw FFI types for the InputEvent enum
// Each of these structs represent one or more InputEvent types,
// they must all include a type id as their first member to identify their
// type
#[repr(C)]
struct MouseMoveEvent {
    type_id: InputEventType,
    x: i32,
    y: i32,
}

#[repr(C)]
struct MouseButtonEvent {
    type_id: InputEventType,
    button: i8,
    x: i32,
    y: i32,
}

#[repr(C)]
struct KeyEvent {
    type_id: InputEventType,
    key: i32,
}

unsafe fn read_input_events(start_ptr: *const c_void, length: usize) -> Vec<InputEvent> {
    let mut buffer = Vec::new();
    let mut offset = 0;
    for _ in 0..length {
        match *(start_ptr.offset(offset) as *const InputEventType) {
            InputEventType::MouseMove => {
                let MouseMoveEvent { x, y, .. } =
                    *(start_ptr.offset(offset) as *const MouseMoveEvent);
                buffer.push(InputEvent::MouseMove(x, y));
                offset += mem::size_of::<MouseMoveEvent>() as isize;
            }
            InputEventType::MouseDown | InputEventType::MouseUp => {
                let MouseButtonEvent {
                    type_id,
                    button,
                    x,
                    y,
                } = *(start_ptr.offset(offset) as *const MouseButtonEvent);
                match type_id {
                    InputEventType::MouseDown => buffer.push(InputEvent::MouseDown {
                        button: button,
                        position: (x, y),
                    }),
                    InputEventType::MouseUp => buffer.push(InputEvent::MouseUp {
                        button: button,
                        position: (x, y),
                    }),
                    _ => {}
                }
                offset += mem::size_of::<MouseButtonEvent>() as isize;
            }
            InputEventType::KeyDown | InputEventType::KeyUp => {
                let KeyEvent { type_id, key } = *(start_ptr.offset(offset) as *const KeyEvent);
                match type_id {
                    InputEventType::KeyDown => buffer.push(InputEvent::KeyDown(key)),
                    InputEventType::KeyUp => buffer.push(InputEvent::KeyUp(key)),
                    _ => {}
                }
                offset += mem::size_of::<KeyEvent>() as isize;
            }
        }
    }
    buffer
}
