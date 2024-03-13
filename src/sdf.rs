use crate::{ffi, tf};
use std::{
    ffi::{c_char, CStr, CString},
    os::raw::c_void,
};

pub struct AssetPath {
    pub(crate) ptr: *mut ffi::sdf_AssetPath_t,
}

impl AssetPath {
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
    pub(crate) ptr: *const ffi::sdf_AssetPath_t,
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

pub struct Layer {
    pub(crate) ptr: *mut ffi::sdf_Layer_t,
}

impl Layer {
    pub fn ptr(&self) -> *mut ffi::sdf_Layer_t {
        self.ptr
    }

    pub fn create_anonymous(name: &str) -> LayerRefPtr {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let name = CString::new(name).unwrap();
            ffi::sdf_Layer_CreateAnonymous(name.as_ptr(), &mut ptr);
            LayerRefPtr { ptr }
        }
    }

    pub fn set_default_prim(&self, name: &tf::Token) {
        unsafe {
            ffi::sdf_Layer_SetDefaultPrim(self.ptr, name.ptr);
        }
    }

    pub fn transfer_content(&self, other: &LayerHandle) {
        unsafe {
            ffi::sdf_Layer_TransferContent(self.ptr, other.ptr);
        }
    }
}

pub struct LayerRef {
    pub(crate) ptr: *mut ffi::sdf_Layer_t,
}

impl LayerRef {
    pub fn from_ptr(ptr: *mut ffi::sdf_Layer_t) -> Self {
        Self { ptr }
    }
}

impl std::ops::Deref for LayerRef {
    type Target = Layer;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const LayerRef as *const Layer) }
    }
}

pub struct LayerHandle {
    pub(crate) ptr: *mut ffi::sdf_LayerHandle_t,
}

pub struct LayerRefPtr {
    pub(crate) ptr: *mut ffi::sdf_LayerRefPtr_t,
}

impl LayerRefPtr {
    pub fn as_handle(&self) -> LayerHandle {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::sdf_LayerRefPtr_as_handle(self.ptr, &mut ptr);
            LayerHandle { ptr }
        }
    }
}

impl std::ops::Deref for LayerRefPtr {
    type Target = Layer;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const LayerRefPtr as *const Layer) }
    }
}

type CanReadFn = extern "C" fn(*const c_char) -> bool;
type ReadFn = extern "C" fn(*mut ffi::sdf_Layer_t, *const c_char, bool) -> bool;
pub use ffi::sdf_Layer_t;
pub fn define_file_format(
    name: &str,
    format_id: &str,
    version: &str,
    target: &str,
    extension: &str,
    fn_can_read: CanReadFn,
    fn_read: ReadFn,
) {
    let name = CString::new(name).unwrap();
    let format_id = CString::new(format_id).unwrap();
    let version = CString::new(version).unwrap();
    let target = CString::new(target).unwrap();
    let extension = CString::new(extension).unwrap();

    unsafe {
        ffi::sdf_define_file_format(
            name.as_ptr(),
            format_id.as_ptr(),
            version.as_ptr(),
            target.as_ptr(),
            extension.as_ptr(),
            fn_can_read as *mut c_void,
            fn_read as *mut c_void,
        );
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
            ffi::sdf_Path_from_string(cs.as_ptr(), &mut ptr);
            Path { ptr }
        }
    }
}

pub struct PathRef {
    pub(crate) ptr: *const ffi::sdf_Path_t,
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
            PathRef { ptr }
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
            tf::TokenRef { ptr }
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
