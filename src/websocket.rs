use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use std::rc::Rc;
use std::cell::RefCell;

type WebSocketOnmessageCallback = unsafe extern "C" fn(*const u8, *mut c_void);
type WebSocketOnopenCallback = unsafe extern "C" fn(*mut c_void);

type SocketId = u32;

extern "C" {
    fn websocket_create(url_ptr: *const u8) -> u32;
    fn websocket_send(handle: u32, data_ptr: *const u8);
    fn websocket_onmessage(handle: u32, fn_ptr: WebSocketOnmessageCallback, arg: *mut c_void);
    fn websocket_onopen(handle: u32, fn_ptr: WebSocketOnopenCallback, arg: *mut c_void);
}

fn set_onopen<T>(socket: SocketId, callback: T)
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

    let callback_ptr = Box::into_raw(Box::new(callback));
    unsafe { websocket_onopen(socket, wrapper::<T>, callback_ptr as *mut c_void) }
}

fn set_onmessage<T>(socket: SocketId, callback: T)
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

    let callback_ptr = Box::into_raw(Box::new(callback));
    unsafe { websocket_onmessage(socket, wrapper::<T>, callback_ptr as *mut c_void) }
}

fn send(handle: SocketId, data: &[u8]) {
    unsafe { websocket_send(handle, data.as_ptr() as *const u8) }
}

type Message = String;

pub struct WebSocket {
    handle: SocketId,
    open: Rc<RefCell<bool>>,
    incoming: Rc<RefCell<Vec<Message>>>,
}

impl WebSocket {
    pub fn connect(url: &str) -> Result<WebSocket, ()> {
        unsafe {
            let handle = websocket_create(url.as_ptr() as *const u8);

            let open = Rc::new(RefCell::new(false));
            let open_cb = Rc::clone(&open);
            set_onopen(handle, move || {
                *open_cb.borrow_mut() = true;
            });

            let incoming = Rc::new(RefCell::new(Vec::new()));
            let incoming_cb = Rc::clone(&incoming);
            set_onmessage(handle, move |msg| {
                incoming_cb.borrow_mut().push(String::from(msg))
            });

            Ok(WebSocket {
                handle: handle,
                open: open,
                incoming: incoming,
            })
        }
    }

    pub fn open(&self) -> bool {
        *self.open.borrow()
    }

    pub fn send(&self, data: &[u8]) -> Result<(), ()> {
        send(self.handle, data);
        Ok(())
    }

    pub fn next<'a>(&'a mut self) -> Option<Message> {
        let mut incoming = self.incoming.borrow_mut();
        let v = incoming.drain(0..).next();
        v
    }
}
