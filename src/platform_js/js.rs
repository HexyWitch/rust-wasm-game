use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use std::ptr::null_mut;
use std::cell::RefCell;

type MainLoopCallback = unsafe extern "C" fn();
type WebSocketOnmessageCallback = unsafe extern "C" fn(*const u8, *mut c_void);
type WebSocketOnopenCallback = unsafe extern "C" fn(*mut c_void);

extern "C" {
    fn js_console_log(ptr: *const u8);
    fn js_set_main_loop(fn_ptr: MainLoopCallback);

    fn js_websocket_create(url_ptr: *const u8) -> u32;
    fn js_websocket_send(handle: u32, data_ptr: *const u8);
    fn js_websocket_onmessage(handle: u32, fn_ptr: WebSocketOnmessageCallback, arg: *mut c_void);
    fn js_websocket_onopen(handle: u32, fn_ptr: WebSocketOnopenCallback, arg: *mut c_void);
    fn js_websocket_close(handle: u32, code: i32, reason_ptr: *const u8);
}

thread_local!(static MAIN_LOOP_CALLBACK: RefCell<*mut c_void> = RefCell::new(null_mut()));

#[allow(dead_code)]
pub fn console_log(s: &str) {
    let c_str = CString::new(s).unwrap();
    unsafe {
        js_console_log(c_str.as_ptr() as *const u8);
    }
}

pub fn set_main_loop_callback<T>(callback: T)
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
        js_set_main_loop(wrapper::<T>);
    }
}

pub type SocketId = u32;

pub fn websocket_create(url: &str) -> SocketId {
    let c_url = CString::new(url).unwrap();
    unsafe { js_websocket_create(c_url.as_ptr() as *const u8) }
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
pub unsafe fn websocket_onopen<T>(socket: SocketId, callback: T) -> CallbackHandle<T>
where
    T: Fn(),
{
    pub unsafe extern "C" fn wrapper<T>(arg: *mut c_void)
    where
        T: Fn(),
    {
        let callback = arg as *mut T;
        (*callback)();
    }

    let handle = CallbackHandle::new(callback);
    js_websocket_onopen(socket, wrapper::<T>, handle.ptr() as *mut c_void);
    handle
}

// sets a websocket's onmessage callback, returns a CallbackHandle which must live as long as the socket
pub unsafe fn websocket_onmessage<T>(socket: SocketId, callback: T) -> CallbackHandle<T>
where
    T: Fn(&str),
{
    pub unsafe extern "C" fn wrapper<T>(msg_ptr: *const u8, arg: *mut c_void)
    where
        T: Fn(&str),
    {
        let msg = CString::from_raw(msg_ptr as *mut c_char);
        let callback = arg as *mut T;
        (*callback)(msg.to_str().unwrap());
    }

    let handle = CallbackHandle::new(callback);
    js_websocket_onmessage(socket, wrapper::<T>, handle.ptr() as *mut c_void);

    handle
}

pub fn websocket_send(handle: SocketId, msg: &str) {
    let c_msg = CString::new(msg).unwrap();
    unsafe { js_websocket_send(handle, c_msg.as_ptr() as *const u8) }
}

pub fn websocket_close(handle: SocketId, code: i32, reason: &str) {
    let c_reason = CString::new(reason).unwrap();
    unsafe { js_websocket_close(handle, code, c_reason.as_ptr() as *const u8) }
}
