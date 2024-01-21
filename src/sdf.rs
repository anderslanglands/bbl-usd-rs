use crate::{ffi, tf};
use std::ffi::{CStr, CString};

pub struct AssetPath {
    pub(crate) ptr: *mut ffi::sdf_AssetPath_t,
}

impl AssetPath {
    pub fn from_path(path: &CStr) -> Self {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::sdf_AssetPath_from_path(path.as_ptr(), &mut ptr);
            Self { ptr }
        }
    }

    pub fn asset_path(&self) -> &str {
        unsafe {
            let mut ptr = std::ptr::null();
            ffi::sdf_AssetPath_GetAssetPath(self.ptr, &mut ptr);
            let cstr = CStr::from_ptr(ptr).to_str().unwrap();
            cstr
        }
    }

    pub fn resolved_path(&self) -> &str {
        unsafe {
            let mut ptr = std::ptr::null();
            ffi::sdf_AssetPath_GetResolvedPath(self.ptr, &mut ptr);
            let cstr = CStr::from_ptr(ptr).to_str().unwrap();
            cstr
        }
    }
}

pub struct AssetPathRef {
    pub(crate) ptr: *mut ffi::sdf_AssetPath_t,
}

impl std::ops::Deref for AssetPathRef {
    type Target = AssetPath;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const AssetPathRef as *const AssetPath) }
    }
}

impl Drop for AssetPath {
    fn drop(&mut self) {
        unsafe {
            ffi::sdf_AssetPath_dtor(self.ptr);
        }
    }
}

pub struct Path {
    pub(crate) ptr: *mut ffi::sdf_Path_t,
}

impl Path {
    pub fn text(&self) -> &'static str {
        unsafe {
            let mut ptr = std::ptr::null();
            ffi::sdf_Path_GetText(self.ptr, &mut ptr);
            CStr::from_ptr(ptr).to_str().unwrap()
        }
    }
}

impl From<&str> for Path {
    fn from(value: &str) -> Self {
        let cs = std::ffi::CString::new(value).unwrap();

        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::sdf_Path_from_string(cs.as_ptr() as *mut i8, &mut ptr);
            Path { ptr }
        }
    }
}

pub struct PathRef {
    pub(crate) ptr: *mut ffi::sdf_Path_t,
}

impl std::ops::Deref for PathRef {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const PathRef as *const Path) }
    }
}

impl Drop for Path {
    fn drop(&mut self) {
        unsafe {
            ffi::sdf_Path_dtor(self.ptr);
        }
    }
}

pub struct PathVector {
    pub(crate) ptr: *mut ffi::sdf_PathVector_t,
}

impl PathVector {
    pub fn size(&self) -> usize {
        unsafe {
            let mut size = 0;
            ffi::sdf_PathVector_size(self.ptr, &mut size);
            size
        }
    }

    pub fn at(&self, index: usize) -> PathRef {
        unsafe {
            let mut ptr = std::ptr::null();
            ffi::sdf_PathVector_op_index(self.ptr, index, &mut ptr);
            PathRef { ptr: ptr as _ }
        }
    }

    pub fn iter(&self) -> PathVectorIterator {
        PathVectorIterator {
            vec: self,
            current: 0,
            end: self.size(),
        }
    }
}

impl Drop for PathVector {
    fn drop(&mut self) {
        unsafe {
            ffi::sdf_PathVector_dtor(self.ptr);
        }
    }
}

impl Default for PathVector {
    fn default() -> Self {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::sdf_PathVector_default(&mut ptr);
            PathVector { ptr }
        }
    }
}

pub struct PathVectorIterator<'a> {
    vec: &'a PathVector,
    current: usize,
    end: usize,
}

impl<'a> Iterator for PathVectorIterator<'a> {
    type Item = PathRef;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            None
        } else {
            let cur = self.current;
            self.current += 1;
            Some(self.vec.at(cur))
        }
    }
}

pub struct ValueTypeName {
    pub(crate) ptr: *mut ffi::sdf_ValueTypeName_t,
}

impl ValueTypeName {
    pub fn as_token(&self) -> tf::Token {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::sdf_ValueTypeName_GetAsToken(self.ptr, &mut ptr);
            tf::Token { ptr }
        }
    }

    pub fn role(&self) -> tf::TokenRef {
        unsafe {
            let mut ptr = std::ptr::null();
            ffi::sdf_ValueTypeName_GetRole(self.ptr, &mut ptr);
            tf::TokenRef { ptr: ptr as _ }
        }
    }
}

impl Drop for ValueTypeName {
    fn drop(&mut self) {
        unsafe {
            ffi::sdf_ValueTypeName_dtor(self.ptr);
        }
    }
}

pub enum Variability {
    Varying,
    Uniform,
}

pub struct LayerOffset {
    pub(crate) ptr: *mut ffi::sdf_LayerOffset_t
}

impl Default for LayerOffset {
    fn default() -> Self {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::sdf_LayerOffset_default(&mut ptr);
            Self {
                ptr
            }
        }
    }
}
