use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use wasm_bindgen::prelude::*;

type WebSocketOnmessageCallback = unsafe extern "C" fn(*const u8, usize, *mut c_void);
type WebSocketOnopenCallback = unsafe extern "C" fn(*mut c_void);

#[wasm_bindgen]
extern "C" {
    fn js_websocket_create(url_ptr: *const c_char, url_len: usize) -> u32;
    fn js_websocket_send(handle: u32, data_ptr: *const u8);
    fn js_websocket_onmessage(
        handle: u32,
        fn_ptr: *const WebSocketOnmessageCallback,
        arg: *mut c_void,
    );
    fn js_websocket_onopen(handle: u32, fn_ptr: *const WebSocketOnopenCallback, arg: *mut c_void);
    fn js_websocket_close(handle: u32, code: i32, reason_ptr: *const c_char, reason_len: usize);
}

pub type SocketId = u32;

pub fn websocket_create(url: &str) -> SocketId {
    let c_url = CString::new(url).unwrap();
    unsafe { js_websocket_create(c_url.as_ptr() as *const c_char, c_url.as_bytes().len()) }
}

pub struct CallbackHandle<T>(*mut T);

impl<T> CallbackHandle<T> {
    fn new(callback: T) -> CallbackHandle<T> {
        let cb_ptr = Box::into_raw(Box::new(callback));
        CallbackHandle(cb_ptr)
    }

    fn ptr(&self) -> *mut T {
        self.0
    }
}

impl<T> Drop for CallbackHandle<T> {
    fn drop(&mut self) {
        unsafe {
            let _callback = Box::<T>::from_raw(self.0);
        }
    }
}

// sets a websocket's onopen callback, returns a CallbackHandle which must live as long as the socket
pub fn websocket_onopen<T>(socket: SocketId, callback: T) -> CallbackHandle<T>
where
    T: Fn() + 'static,
{
    pub unsafe extern "C" fn wrapper<T>(arg: *mut c_void)
    where
        T: Fn(),
    {
        let callback = arg as *mut T;
        (*callback)();
    }

    let handle = CallbackHandle::new(callback);
    unsafe {
        js_websocket_onopen(
            socket,
            wrapper::<T> as *const WebSocketOnopenCallback,
            handle.ptr() as *mut c_void,
        );
    }
    handle
}

// sets a websocket's onmessage callback, returns a CallbackHandle which must live as long as the socket
pub fn websocket_onmessage<T>(socket: SocketId, callback: T) -> CallbackHandle<T>
where
    T: Fn(Vec<u8>) + 'static,
{
    pub unsafe extern "C" fn wrapper<T>(data_ptr: *const u8, data_len: usize, arg: *mut c_void)
    where
        T: Fn(Vec<u8>),
    {
        // This relies on the memory being previously allocated using the exported alloc function
        // which creates a Vec
        let data = Vec::from_raw_parts(data_ptr as *mut u8, data_len, data_len);
        let callback = arg as *mut T;
        (*callback)(data);
    }

    let handle = CallbackHandle::new(callback);
    unsafe {
        js_websocket_onmessage(
            socket,
            wrapper::<T> as *const WebSocketOnmessageCallback,
            handle.ptr() as *mut c_void,
        );
    }
    handle
}

pub fn websocket_send(handle: SocketId, data: &[u8]) {
    unsafe { js_websocket_send(handle, data.as_ptr()) }
}

pub fn websocket_close(handle: SocketId, code: i32, reason: &str) {
    let c_reason = CString::new(reason).unwrap();
    unsafe {
        js_websocket_close(
            handle,
            code,
            c_reason.as_ptr() as *const c_char,
            c_reason.as_bytes().len(),
        )
    }
}
