#![feature(set_stdio)]

use std::mem;
use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use std::io;

extern "C" {
    fn write(ptr: *const u8);
    fn websocket_create(url_ptr: *const u8) -> u32;
    fn websocket_send(handle: u32, data_ptr: *const u8) -> bool;
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

#[no_mangle]
pub extern "C" fn run() {
    io::set_print(Some(Box::new(ConsoleWriter(Vec::new()))));
    io::set_panic(Some(Box::new(ConsoleWriter(Vec::new()))));

    println!("Running client");
    let url = CString::new("ws://localhost:3012").unwrap();
    let socket;
    unsafe {
        println!("Creating websocket");
        socket = websocket_create(url.into_raw() as *const u8);
    }
    println!("Created socket with handle: {}", socket);

    let message = CString::new("Testing sending a message").unwrap();
    unsafe {
        websocket_send(socket, message.into_raw() as *const u8);
    }
}
