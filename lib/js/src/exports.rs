use std::os::raw::{c_void};
use std::mem;

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}

#[no_mangle]
pub extern "C" fn dealloc(data_ptr: *mut c_void, size: usize) {
    unsafe {
        let _buf = Vec::from_raw_parts(data_ptr, 0, size);
    }
}
