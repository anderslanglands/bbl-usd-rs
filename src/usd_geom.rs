use crate::ffi;
use crate::sdf;
use crate::usd;

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
