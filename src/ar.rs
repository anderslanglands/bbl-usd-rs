use crate::{cpp, ffi};
use std::ffi::CString;

pub struct ResolvedPath {
    pub(crate) ptr: *mut ffi::ar_ResolvedPath_t,
}

impl ResolvedPath {
    pub fn new(string: &cpp::String) -> Self {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::ar_ResolvedPath_ctor(string.ptr, &mut ptr);
            Self { ptr }
        }
    }

    pub fn is_empty(&self) -> bool {
        unsafe {
            let mut is_empty = false;
            ffi::ar_ResolvedPath_IsEmpty(self.ptr, &mut is_empty);
            is_empty
        }
    }

    pub fn get_path_string(&self) -> cpp::StringRef {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::ar_ResolvedPath_GetPathString(
                self.ptr,
                (&mut ptr) as *mut *mut ffi::std_String_t as _,
            );
            cpp::StringRef { ptr }
        }
    }

    pub fn ptr(&self) -> *const ffi::ar_ResolvedPath_t {
        self.ptr
    }
}

impl Drop for ResolvedPath {
    fn drop(&mut self) {
        unsafe {
            ffi::ar_ResolvedPath_dtor(self.ptr);
        }
    }
}

pub struct ResolvedPathRef {
    pub(crate) ptr: *mut ffi::ar_ResolvedPath_t,
}

impl ResolvedPathRef {
    pub fn from_raw(ptr: *const ffi::ar_ResolvedPath_t) -> Self {
        Self { ptr: ptr as _ }
    }
}

impl std::ops::Deref for ResolvedPathRef {
    type Target = ResolvedPath;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const ResolvedPathRef as *const ResolvedPath) }
    }
}

pub struct Timestamp {
    pub(crate) ptr: *mut ffi::ar_Timestamp_t,
}

impl Timestamp {
    pub fn from_time(time: f64) -> Self {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::ar_Timestamp_from_time(time, &mut ptr);
            Self { ptr }
        }
    }

    pub fn ptr(&self) -> *const ffi::ar_Timestamp_t {
        self.ptr
    }
}

impl Drop for Timestamp {
    fn drop(&mut self) {
        unsafe {
            ffi::ar_Timestamp_dtor(self.ptr);
        }
    }
}
