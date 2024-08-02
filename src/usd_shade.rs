use crate::ffi;
use crate::tf;
use crate::usd;
use crate::sdf;

pub struct Material {
    pub(crate) ptr: *mut ffi::usdShade_Material_t,
}

impl Material {
    pub fn from_prim(prim: &usd::Prim) -> Self {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usdShade_Material_from_prim(prim.ptr, &mut ptr);
            Material{
                ptr
            }
        }
    }
    
    pub fn compute_surface_source(self, context_vector: tf::TokenVector, source_name: tf::Token, source_type:  AttributeType  ) -> Shader {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usdShade_Material_ComputeSurfaceSource_01(self.ptr, context_vector.ptr,source_name.ptr, source_type.ptr, &mut ptr);
            Shader{
                ptr
            }
        }
    }

    pub fn get_surface_attr(self) -> usd::Attribute {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usdShade_Material_GetSurfaceAttr(self.ptr, &mut ptr);
            usd::Attribute{
                ptr
            }
        }
    }
    
}

pub struct MaterialBindingAPIDirectBinding {
    pub(crate) ptr: *mut ffi::usdShade_MaterialBindingAPIDirectBinding_t,
} 

impl MaterialBindingAPIDirectBinding{
    pub fn from_schema(binding_rel: &usd::Relationship) -> Self {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usdShade_MaterialBindingAPIDirectBinding_from_schema(binding_rel.ptr as *const ffi::usd_Relationship_t, &mut ptr);
            MaterialBindingAPIDirectBinding{
                ptr
            }
        }
    }
    
    pub fn get_material_path(&self) -> sdf::Path {
        unsafe {
            let mut ptr: *mut *const ffi::sdf_Path_t = std::ptr::null_mut();
            ffi::usdShade_MaterialBindingAPIDirectBinding_GetMaterialPath(self.ptr, ptr);
            let mut_ptr = *ptr as *mut ffi::sdf_Path_t;
            sdf::Path{ ptr: mut_ptr }
        }
    }
    
    pub fn is_bound(&self) -> bool {
        unsafe {
            let mut result = false;
            ffi::usdShade_MaterialBindingAPIDirectBinding_IsBound(self.ptr, &mut result);
            result
        }
    }
}

pub struct Shader {
    pub(crate) ptr: *mut ffi::usdShade_Shader_t,
}

impl Shader {
    pub fn from_prim(prim: &usd::Prim) -> Self {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usdShade_Shader_from_prim(prim.ptr, &mut ptr);
            Shader{
                ptr
            }
        }
    }
    
    pub fn get_id_attr(self) -> usd::Attribute {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usdShade_Shader_GetIdAttr(self.ptr, &mut ptr);
            usd::Attribute{
                ptr
            }
        }
    }
    
    pub fn get_prim(&self) -> usd::Prim {
        unsafe {
            let ptr = std::ptr::null_mut();
            ffi::usdShade_Shader_GetPrim(self.ptr, ptr);
            usd::Prim { ptr: *ptr }
        }
    }
    
}

pub struct AttributeType {
    pub(crate) ptr: *mut ffi::usdShade_AttributeType,
}

impl AttributeType{
     pub fn default() -> Self {
         let ptr = Box::into_raw( Box::new(0 as i32) );
            unsafe {
                AttributeType{
                    ptr:  ptr as *mut ffi::usdShade_AttributeType
                }
            }
    }
}
