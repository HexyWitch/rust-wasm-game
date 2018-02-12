use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn alloc_str(size: usize) -> *mut c_char {
    let buf = CString::new(vec![0; size]).unwrap();
    return buf.into_raw();
}

#[no_mangle]
pub extern "C" fn dealloc_str(ptr: *mut c_char) {
    unsafe {
        let _buf = CString::from_raw(ptr);
    }
}
