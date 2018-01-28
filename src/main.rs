#![feature(set_stdio)]
#![feature(conservative_impl_trait)]

mod websocket;

use std::cell::RefCell;
use std::mem;
use std::ffi::CString;
use std::ptr::null_mut;
use std::os::raw::{c_char, c_void};
use std::io;

use websocket::WebSocket;
type MainLoopCallback = unsafe extern "C" fn();

extern "C" {
    fn write(ptr: *const u8);

    fn set_main_loop(fn_ptr: MainLoopCallback);
}

// In order to work with the memory we expose (de)allocation methods
#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut c_void, cap: usize) {
    unsafe {
        let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}

#[no_mangle]
pub extern "C" fn dealloc_str(ptr: *mut c_char) {
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}

struct ConsoleWriter(Vec<u8>);
impl std::io::Write for ConsoleWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        for i in buf {
            self.0.push(*i);
            if *i == '\n' as u8 {
                let buf = mem::replace(&mut self.0, Vec::new());
                let c_str = CString::new(buf).unwrap();
                unsafe {
                    write(c_str.into_raw() as *const u8);
                }
            }
        }
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        let buf = mem::replace(&mut self.0, Vec::new());
        let c_str = CString::new(buf).unwrap();
        unsafe {
            write(c_str.into_raw() as *const u8);
        }
        Ok(())
    }
}

thread_local!(static MAIN_LOOP_CALLBACK: RefCell<*mut c_void> = RefCell::new(null_mut()));

fn set_main_loop_callback<T>(callback: T)
where
    T: FnMut(),
{
    MAIN_LOOP_CALLBACK
        .with(|cb| *cb.borrow_mut() = Box::into_raw(Box::new(callback)) as *mut c_void);

    pub unsafe extern "C" fn wrapper<T>()
    where
        T: FnMut(),
    {
        MAIN_LOOP_CALLBACK.with(|cb| {
            let callback = *cb.borrow_mut() as *mut T;
            (*callback)()
        })
    }

    unsafe {
        set_main_loop(wrapper::<T>);
    }
}

fn main() {}

#[no_mangle]
pub extern "C" fn run() {
    io::set_print(Some(Box::new(ConsoleWriter(Vec::new()))));
    io::set_panic(Some(Box::new(ConsoleWriter(Vec::new()))));

    let mut socket = WebSocket::connect("ws://localhost:3012").unwrap();
    let mut ping = true;
    let mut x = 1;
    let update = || {
        if socket.open() {
            if ping {
                let msg = format!("{}", x);
                println!("Ping: {}", x);
                socket.send(msg.as_bytes()).unwrap();
                ping = false;
                x += 1;
            }

            while let Some(msg) = socket.next() {
                println!("Pong: {}", msg);
                ping = true;
            }
        }
    };

    set_main_loop_callback(update);
}
