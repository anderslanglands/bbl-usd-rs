use crate::ffi;
use std::ffi::{CStr, CString};

pub struct String {
    ptr: *mut ffi::std_String_t,
}

impl String {
    pub fn as_str(&self) -> &str {
        unsafe {
            let mut ptr = std::ptr::null();
            ffi::std_String_c_str(self.ptr, &mut ptr);
            let cstr = CStr::from_ptr(ptr).to_str().unwrap();
            cstr
        }
    }
}

pub struct StringRef {
    pub(crate) ptr: *mut ffi::std_String_t,
}

impl std::ops::Deref for StringRef {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const StringRef as *const String) }
    }
}

impl Drop for String {
    fn drop(&mut self) {
        unsafe {
            ffi::std_String_dtor(self.ptr);
        }
    }
}


