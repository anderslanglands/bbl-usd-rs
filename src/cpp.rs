use crate::ffi;
use std::ffi::{CStr, CString};

pub struct String {
    pub(crate) ptr: *mut ffi::std_String_t,
}

impl String {
    pub fn new(string: &CStr) -> Self {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::std_String_from_char_ptr(string.as_ptr(), &mut ptr);
            Self { ptr }
        }
    }

    pub fn as_str(&self) -> &str {
        unsafe {
            let mut ptr = std::ptr::null();
            ffi::std_String_c_str(self.ptr, &mut ptr);
            let cstr = CStr::from_ptr(ptr).to_str().unwrap();
            cstr
        }
    }

    pub fn ptr(&self) -> *const ffi::std_String_t {
        self.ptr
    }
}

impl Drop for String {
    fn drop(&mut self) {
        unsafe {
            ffi::std_String_dtor(self.ptr);
        }
    }
}

pub struct StringRef {
    pub(crate) ptr: *mut ffi::std_String_t,
}

impl StringRef {
    pub fn from_ptr(ptr: *const ffi::std_String_t) -> Self {
        Self { ptr: ptr as _ }
    }
}

impl std::ops::Deref for StringRef {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const StringRef as *const String) }
    }
}
