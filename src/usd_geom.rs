use crate::ffi;
use crate::sdf;
use crate::usd;
use crate::vt;
use crate::tf;

pub struct Mesh {
    pub(crate) ptr: *mut ffi::usdGeom_Mesh_t,
}

impl Mesh {
    pub fn define(stage: &usd::StageWeakPtr, path: &sdf::Path) -> Mesh {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usdGeom_Mesh_Define(stage.ptr, path.ptr, &mut ptr);
            Mesh { ptr }
        }
    }

    pub fn points_attr(&self) -> usd::Attribute {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usdGeom_Mesh_GetPointsAttr(self.ptr, &mut ptr);
            usd::Attribute { ptr }
        }
    }

    pub fn extent_attr(&self) -> usd::Attribute {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usdGeom_Mesh_GetExtentAttr(self.ptr, &mut ptr);
            usd::Attribute { ptr }
        }
    }

    pub fn face_vertex_counts_attr(&self) -> usd::Attribute {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usdGeom_Mesh_GetFaceVertexCountsAttr(self.ptr, &mut ptr);
            usd::Attribute { ptr }
        }
    }

    pub fn face_vertex_indices_attr(&self) -> usd::Attribute {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usdGeom_Mesh_GetFaceVertexIndicesAttr(self.ptr, &mut ptr);
            usd::Attribute { ptr }
        }
    }
}

pub struct XformCache {
    pub(crate) ptr: *mut ffi::usdGeom_XformCache_t,
}

impl XformCache {
    pub fn new(time: usd::TimeCode) -> XformCache {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usdGeom_XformCache_new( time.0 , &mut ptr);
            XformCache { ptr }
        }
    }
    
    pub fn get_local_to_world_transform(&self, prim: &usd::Prim) -> vt::DMat4 {
        unsafe {
            let ptr  = Box::into_raw(Box::new(vt::DMat4::IDENTITY)) as *mut ffi::gf_Matrix4d_t;
            ffi::usdGeom_XformCache_GetLocalToWorldTransform(self.ptr, prim.ptr,   ptr);
            *Box::from_raw(ptr as *mut vt::DMat4)
        }
    }
}

pub struct Primvar {
    pub(crate) ptr: *mut ffi::usdGeom_Primvar_t,
}

impl Primvar {
    pub fn new(attr: &usd::Attribute) -> Self  {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usdGeom_Primvar_new(attr.ptr, &mut ptr);
            Primvar { ptr }
        }
    }
    
    pub fn get_interpolation(&self) -> tf::Token {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usdGeom_Primvar_GetInterpolation(self.ptr, &mut ptr);
            tf::Token { ptr }
        }
    }
    
}