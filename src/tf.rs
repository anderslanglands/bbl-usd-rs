use crate::ffi;
use std::ffi::{CStr, CString};
use std::fmt;

pub struct Token {
    pub(crate) ptr: *mut ffi::tf_Token_t,
}

impl Token {
    pub fn new(text: &str) -> Token {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let text = CString::new(text).unwrap();
            ffi::tf_Token_new(text.as_ptr(), &mut ptr);
            std::mem::forget(text);
            Token { ptr }
        }
    }

    pub fn text(&self) -> &'static str {
        unsafe {
            let mut ptr = std::ptr::null();
            ffi::tf_Token_GetText(self.ptr, &mut ptr);
            CStr::from_ptr(ptr).to_str().unwrap()
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text())
    }
}

impl Drop for Token {
    fn drop(&mut self) {
        unsafe {
            ffi::tf_Token_dtor(self.ptr);
        }
    }
}

impl AsRef<str> for Token {
    fn as_ref(&self) -> &str {
        self.text()
    }
}

pub struct TokenRef {
    pub(crate) ptr: *const ffi::tf_Token_t,
}

impl std::ops::Deref for TokenRef {
    type Target = Token;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const TokenRef as *const Token) }
    }
}

impl fmt::Display for TokenRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text())
    }
}

impl AsRef<str> for TokenRef {
    fn as_ref(&self) -> &str {
        self.text()
    }
}

