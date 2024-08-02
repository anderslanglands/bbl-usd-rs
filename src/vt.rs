use std::ops::IndexMut;

use crate::{ffi, sdf, tf};
pub use glam::{DVec3, Vec2, Vec3, Vec4, Mat2, DMat2, Mat3, DMat3, Mat4, DMat4};

pub struct TokenArray {
    pub(crate) ptr: *mut ffi::vt_TokenArray_t,
}

impl TokenArray {
    pub fn size(&self) -> usize {
        unsafe {
            let mut result = 0;
            ffi::vt_TokenArray_size(self.ptr, &mut result);
            result
        }
    }

    pub fn at(&self, index: usize) -> tf::TokenRef {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::vt_TokenArray_op_index(self.ptr, index, &mut ptr);
            tf::TokenRef { ptr }
        }
    }

    pub fn iter(&self) -> TokenArrayIterator {
        TokenArrayIterator {
            vec: self,
            current: 0,
            end: self.size(),
        }
    }
}

impl<'a> IntoIterator for &'a TokenArray {
    type Item = tf::TokenRef;
    type IntoIter = TokenArrayIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct TokenArrayIterator<'a> {
    vec: &'a TokenArray,
    current: usize,
    end: usize,
}

impl<'a> Iterator for TokenArrayIterator<'a> {
    type Item = tf::TokenRef;

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

pub struct TokenArrayRef {
    pub(crate) ptr: *const ffi::vt_TokenArray_t,
}

impl std::ops::Deref for TokenArrayRef {
    type Target = TokenArray;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const TokenArrayRef as *const TokenArray) }
    }
}

pub struct IntArray {
    pub(crate) ptr: *mut ffi::vt_IntArray_t,
}

impl IntArray {
    pub fn new(values: &[i32]) -> Self {
        unsafe {
            let range = values.as_ptr_range();
            let mut ptr = std::ptr::null_mut();
            ffi::vt_IntArray_from_range(range.start, range.end, &mut ptr);
            IntArray { ptr }
        }
    }

    pub fn size(&self) -> usize {
        unsafe {
            let mut result = 0;
            ffi::vt_IntArray_size(self.ptr, &mut result);
            result
        }
    }

    pub fn at(&self, index: usize) -> &i32 {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::vt_IntArray_op_index(self.ptr, index, &mut ptr);
            &*(ptr as *mut i32)
        }
    }

    pub fn iter(&self) -> IntArrayIterator {
        IntArrayIterator {
            vec: self,
            current: 0,
            end: self.size(),
        }
    }
}

impl<'a> IntoIterator for &'a IntArray {
    type Item = &'a i32;
    type IntoIter = IntArrayIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct IntArrayIterator<'a> {
    vec: &'a IntArray,
    current: usize,
    end: usize,
}

impl<'a> Iterator for IntArrayIterator<'a> {
    type Item = &'a i32;

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

pub struct IntArrayRef {
    pub(crate) ptr: *const ffi::vt_IntArray_t,
}

impl std::ops::Deref for IntArrayRef {
    type Target = IntArray;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const IntArrayRef as *const IntArray) }
    }
}

pub struct FloatArray {
    pub(crate) ptr: *mut ffi::vt_FloatArray_t,
}

impl FloatArray {
    pub fn size(&self) -> usize {
        unsafe {
            let mut result = 0;
            ffi::vt_FloatArray_size(self.ptr, &mut result);
            result
        }
    }

    pub fn at(&self, index: usize) -> &f32 {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::vt_FloatArray_op_index(self.ptr, index, &mut ptr);
            &*(ptr as *mut f32)
        }
    }

    pub fn iter(&self) -> FloatArrayIterator {
        FloatArrayIterator {
            vec: self,
            current: 0,
            end: self.size(),
        }
    }
}

impl<'a> IntoIterator for &'a FloatArray {
    type Item = &'a f32;
    type IntoIter = FloatArrayIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct FloatArrayIterator<'a> {
    vec: &'a FloatArray,
    current: usize,
    end: usize,
}

impl<'a> Iterator for FloatArrayIterator<'a> {
    type Item = &'a f32;

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

pub struct FloatArrayRef {
    pub(crate) ptr: *const ffi::vt_FloatArray_t,
}

impl std::ops::Deref for FloatArrayRef {
    type Target = FloatArray;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const FloatArrayRef as *const FloatArray) }
    }
}

pub struct DoubleArray {
    pub(crate) ptr: *mut ffi::vt_DoubleArray_t,
}

impl DoubleArray {
    pub fn size(&self) -> usize {
        unsafe {
            let mut result = 0;
            ffi::vt_DoubleArray_size(self.ptr, &mut result);
            result
        }
    }

    pub fn at(&self, index: usize) -> &f64 {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::vt_DoubleArray_op_index(self.ptr, index, &mut ptr);
            &*(ptr as *mut f64)
        }
    }

    pub fn iter(&self) -> DoubleArrayIterator {
        DoubleArrayIterator {
            vec: self,
            current: 0,
            end: self.size(),
        }
    }
}

impl<'a> IntoIterator for &'a DoubleArray {
    type Item = &'a f64;
    type IntoIter = DoubleArrayIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct DoubleArrayIterator<'a> {
    vec: &'a DoubleArray,
    current: usize,
    end: usize,
}

impl<'a> Iterator for DoubleArrayIterator<'a> {
    type Item = &'a f64;

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

pub struct DoubleArrayRef {
    pub(crate) ptr: *const ffi::vt_DoubleArray_t,
}

impl std::ops::Deref for DoubleArrayRef {
    type Target = DoubleArray;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const DoubleArrayRef as *const DoubleArray) }
    }
}

pub struct Vec2Array {
    pub(crate) ptr: *mut ffi::gf_Vec2fArray_t,
}

impl Vec2Array {
    pub fn size(&self) -> usize {
        unsafe {
            let mut result = 0;
            ffi::gf_Vec2fArray_size(self.ptr, &mut result);
            result
        }
    }

    pub fn at(&self, index: usize) -> &Vec2 {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::gf_Vec2fArray_op_index(self.ptr, index, &mut ptr);
            &*(ptr as *mut Vec2)
        }
    }

    pub fn iter(&self) -> Vec2ArrayIterator {
        Vec2ArrayIterator {
            vec: self,
            current: 0,
            end: self.size(),
        }
    }
}

impl<'a> IntoIterator for &'a Vec2Array {
    type Item = &'a Vec2;
    type IntoIter = Vec2ArrayIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Vec2ArrayIterator<'a> {
    vec: &'a Vec2Array,
    current: usize,
    end: usize,
}

impl<'a> Iterator for Vec2ArrayIterator<'a> {
    type Item = &'a Vec2;

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

pub struct Vec2ArrayRef {
    pub(crate) ptr: *const ffi::gf_Vec2fArray_t,
}

impl std::ops::Deref for Vec2ArrayRef {
    type Target = Vec2Array;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Vec2ArrayRef as *const Vec2Array) }
    }
}

pub struct Vec3Array {
    pub(crate) ptr: *mut ffi::gf_Vec3fArray_t,
}

impl Vec3Array {
    pub fn new(data: &[Vec3]) -> Self {
        unsafe {
            let range = data.as_ptr_range();
            let mut ptr = std::ptr::null_mut();
            ffi::gf_Vec3fArray_from_range(
                range.start as *const ffi::gf_Vec3f_t,
                range.end as *const ffi::gf_Vec3f_t,
                &mut ptr,
            );
            Vec3Array { ptr }
        }
    }

    pub fn size(&self) -> usize {
        unsafe {
            let mut result = 0;
            ffi::gf_Vec3fArray_size(self.ptr, &mut result);
            result
        }
    }

    pub fn at(&self, index: usize) -> &Vec3 {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::gf_Vec3fArray_op_index(self.ptr, index, &mut ptr);
            &*(ptr as *mut Vec3)
        }
    }

    pub fn iter(&self) -> Vec3ArrayIterator {
        Vec3ArrayIterator {
            vec: self,
            current: 0,
            end: self.size(),
        }
    }
}

impl<'a> IntoIterator for &'a Vec3Array {
    type Item = &'a Vec3;
    type IntoIter = Vec3ArrayIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Vec3ArrayIterator<'a> {
    vec: &'a Vec3Array,
    current: usize,
    end: usize,
}

impl<'a> Iterator for Vec3ArrayIterator<'a> {
    type Item = &'a Vec3;

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

pub struct Vec3ArrayRef {
    pub(crate) ptr: *const ffi::gf_Vec3fArray_t,
}

impl std::ops::Deref for Vec3ArrayRef {
    type Target = Vec3Array;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Vec3ArrayRef as *const Vec3Array) }
    }
}

pub struct Vec4Array {
    pub(crate) ptr: *mut ffi::gf_Vec4fArray_t,
}

impl Vec4Array {
    pub fn size(&self) -> usize {
        unsafe {
            let mut result = 0;
            ffi::gf_Vec4fArray_size(self.ptr, &mut result);
            result
        }
    }

    pub fn at(&self, index: usize) -> &Vec4 {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::gf_Vec4fArray_op_index(self.ptr, index, &mut ptr);
            &*(ptr as *mut Vec4)
        }
    }

    pub fn iter(&self) -> Vec4ArrayIterator {
        Vec4ArrayIterator {
            vec: self,
            current: 0,
            end: self.size(),
        }
    }
}

impl<'a> IntoIterator for &'a Vec4Array {
    type Item = &'a Vec4;
    type IntoIter = Vec4ArrayIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Vec4ArrayIterator<'a> {
    vec: &'a Vec4Array,
    current: usize,
    end: usize,
}

impl<'a> Iterator for Vec4ArrayIterator<'a> {
    type Item = &'a Vec4;

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

pub struct Vec4ArrayRef {
    pub(crate) ptr: *const ffi::gf_Vec4fArray_t,
}

impl std::ops::Deref for Vec4ArrayRef {
    type Target = Vec4Array;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Vec4ArrayRef as *const Vec4Array) }
    }
}

pub struct Value {
    pub(crate) ptr: *mut ffi::vt_Value_t,
}

impl Value {
    pub fn get<T: ValueMember>(&self) -> Option<&T> {
        T::get(self)
    }
}

impl Value {
    pub fn as_token(&self) -> Option<tf::TokenRef> {
        unsafe {
            let mut is_holding = false;
            ffi::vt_Value_IsHolding_TfToken(self.ptr, &mut is_holding);
            if is_holding {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_TfToken(self.ptr, &mut ptr);
                Some(tf::TokenRef { ptr })
            } else {
                None
            }
        }
    }

    pub fn as_token_array(&self) -> Option<TokenArrayRef> {
        unsafe {
            let mut is_holding = false;
            ffi::vt_Value_IsHolding_VtTokenArray(self.ptr, &mut is_holding);
            if is_holding {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_VtTokenArray(self.ptr, &mut ptr);
                Some(TokenArrayRef { ptr })
            } else {
                None
            }
        }
    }

    pub fn as_int_array(&self) -> Option<IntArrayRef> {
        unsafe {
            let mut is_holding = false;
            ffi::vt_Value_IsHolding_VtIntArray(self.ptr, &mut is_holding);
            if is_holding {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_VtIntArray(self.ptr, &mut ptr);
                Some(IntArrayRef { ptr })
            } else {
                None
            }
        }
    }

    pub fn as_float_array(&self) -> Option<FloatArrayRef> {
        unsafe {
            let mut is_holding = false;
            ffi::vt_Value_IsHolding_VtFloatArray(self.ptr, &mut is_holding);
            if is_holding {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_VtFloatArray(self.ptr, &mut ptr);
                Some(FloatArrayRef { ptr })
            } else {
                None
            }
        }
    }

    pub fn as_double_array(&self) -> Option<DoubleArrayRef> {
        unsafe {
            let mut is_holding = false;
            ffi::vt_Value_IsHolding_VtDoubleArray(self.ptr, &mut is_holding);
            if is_holding {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_VtDoubleArray(self.ptr, &mut ptr);
                Some(DoubleArrayRef { ptr })
            } else {
                None
            }
        }
    }

    pub fn as_vec2_array(&self) -> Option<Vec2ArrayRef> {
        unsafe {
            let mut is_holding = false;
            ffi::vt_Value_IsHolding_VtVec2fArray(self.ptr, &mut is_holding);
            if is_holding {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_VtVec2fArray(self.ptr, &mut ptr);
                Some(Vec2ArrayRef { ptr })
            } else {
                None
            }
        }
    }

    pub fn as_vec3_array(&self) -> Option<Vec3ArrayRef> {
        unsafe {
            let mut is_holding = false;
            ffi::vt_Value_IsHolding_VtVec3fArray(self.ptr, &mut is_holding);
            if is_holding {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_VtVec3fArray(self.ptr, &mut ptr);
                Some(Vec3ArrayRef { ptr })
            } else {
                None
            }
        }
    }

    pub fn as_vec4_array(&self) -> Option<Vec4ArrayRef> {
        unsafe {
            let mut is_holding = false;
            ffi::vt_Value_IsHolding_VtVec4fArray(self.ptr, &mut is_holding);
            if is_holding {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_VtVec4fArray(self.ptr, &mut ptr);
                Some(Vec4ArrayRef { ptr })
            } else {
                None
            }
        }
    }

    pub fn as_asset_path(&self) -> Option<sdf::AssetPathRef> {
        unsafe {
            let mut is_holding = false;
            ffi::vt_Value_IsHolding_SdfAssetPath(self.ptr, &mut is_holding);
            if is_holding {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_SdfAssetPath(self.ptr, &mut ptr);
                Some(sdf::AssetPathRef { ptr })
            } else {
                None
            }
        }
    }
}

pub struct ValueRef {
    pub(crate) ptr: *mut ffi::vt_Value_t,
}

impl std::ops::Deref for ValueRef {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const ValueRef as *const Value) }
    }
}

impl Drop for Value {
    fn drop(&mut self) {
        unsafe {
            ffi::vt_Value_dtor(self.ptr);
        }
    }
}

pub trait ValueMember {
    fn get(value: &Value) -> Option<&Self>;
    fn is_holding(value: &Value) -> bool;
    fn as_value(member: &Self) -> Value;
}

impl ValueMember for i32 {
    fn get(value: &Value) -> Option<&Self> {
        if Self::is_holding(value) {
            unsafe {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_int(value.ptr, &mut ptr);
                Some(&*ptr)
            }
        } else {
            None
        }
    }

    fn is_holding(value: &Value) -> bool {
        unsafe {
            let mut result = false;
            ffi::vt_Value_IsHolding_int(value.ptr, &mut result);
            result
        }
    }

    fn as_value(member: &Self) -> Value {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::vt_Value_from_int(*member, &mut ptr);
            Value { ptr }
        }
    }
}

impl ValueMember for IntArray {
    fn get(value: &Value) -> Option<&Self> {
        if Self::is_holding(value) {
            unsafe {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_VtIntArray(value.ptr, &mut ptr);
                Some(&*(ptr as *mut IntArray))
            }
        } else {
            None
        }
    }

    fn is_holding(value: &Value) -> bool {
        unsafe {
            let mut result = false;
            ffi::vt_Value_IsHolding_VtIntArray(value.ptr, &mut result);
            result
        }
    }

    fn as_value(member: &Self) -> Value {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::vt_Value_from_VtIntArray(member.ptr, &mut ptr);
            Value { ptr }
        }
    }
}

impl ValueMember for f32 {
    fn get(value: &Value) -> Option<&Self> {
        if Self::is_holding(value) {
            unsafe {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_float(value.ptr, &mut ptr);
                Some(&*ptr)
            }
        } else {
            None
        }
    }

    fn is_holding(value: &Value) -> bool {
        unsafe {
            let mut result = false;
            ffi::vt_Value_IsHolding_float(value.ptr, &mut result);
            result
        }
    }

    fn as_value(member: &Self) -> Value {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::vt_Value_from_float(*member, &mut ptr);
            Value { ptr }
        }
    }
}

impl ValueMember for f64 {
    fn get(value: &Value) -> Option<&Self> {
        if Self::is_holding(value) {
            unsafe {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_double(value.ptr, &mut ptr);
                Some(&*ptr)
            }
        } else {
            None
        }
    }

    fn is_holding(value: &Value) -> bool {
        unsafe {
            let mut result = false;
            ffi::vt_Value_IsHolding_double(value.ptr, &mut result);
            result
        }
    }

    fn as_value(member: &Self) -> Value {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::vt_Value_from_double(*member, &mut ptr);
            Value { ptr }
        }
    }
}

impl ValueMember for bool {
    fn get(value: &Value) -> Option<&Self> {
        if Self::is_holding(value) {
            unsafe {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_bool(value.ptr, &mut ptr);
                Some(&*ptr)
            }
        } else {
            None
        }
    }

    fn is_holding(value: &Value) -> bool {
        unsafe {
            let mut result = false;
            ffi::vt_Value_IsHolding_bool(value.ptr, &mut result);
            result
        }
    }

    fn as_value(member: &Self) -> Value {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::vt_Value_from_bool(*member, &mut ptr);
            Value { ptr }
        }
    }
}

impl ValueMember for Vec2 {
    fn get(value: &Value) -> Option<&Self> {
        if Self::is_holding(value) {
            unsafe {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_GfVec2f(value.ptr, &mut ptr);
                Some(&*(ptr as *mut Vec2))
            }
        } else {
            None
        }
    }

    fn is_holding(value: &Value) -> bool {
        unsafe {
            let mut result = false;
            ffi::vt_Value_IsHolding_GfVec2f(value.ptr, &mut result);
            result
        }
    }

    fn as_value(member: &Self) -> Value {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::vt_Value_from_GfVec2f(
                *(member as *const Vec2 as *const ffi::gf_Vec2f_t),
                &mut ptr,
            );
            Value { ptr }
        }
    }
}

impl ValueMember for Vec3 {
    fn get(value: &Value) -> Option<&Self> {
        if Self::is_holding(value) {
            unsafe {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_GfVec3f(value.ptr, &mut ptr);
                Some(&*(ptr as *mut Vec3))
            }
        } else {
            None
        }
    }

    fn is_holding(value: &Value) -> bool {
        unsafe {
            let mut result = false;
            ffi::vt_Value_IsHolding_GfVec3f(value.ptr, &mut result);
            result
        }
    }

    fn as_value(member: &Self) -> Value {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::vt_Value_from_GfVec3f(
                *(member as *const Vec3 as *const ffi::gf_Vec3f_t),
                &mut ptr,
            );
            Value { ptr }
        }
    }
}

impl ValueMember for Vec3Array {
    fn get(value: &Value) -> Option<&Self> {
        if Self::is_holding(value) {
            unsafe {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_VtVec3fArray(value.ptr, &mut ptr);
                Some(&*(ptr as *mut Vec3Array))
            }
        } else {
            None
        }
    }

    fn is_holding(value: &Value) -> bool {
        unsafe {
            let mut result = false;
            ffi::vt_Value_IsHolding_VtVec3fArray(value.ptr, &mut result);
            result
        }
    }

    fn as_value(member: &Self) -> Value {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::vt_Value_from_VtVec3fArray(member.ptr, &mut ptr);
            Value { ptr }
        }
    }
}

impl ValueMember for DVec3 {
    fn get(value: &Value) -> Option<&Self> {
        if Self::is_holding(value) {
            unsafe {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_GfVec3d(value.ptr, &mut ptr);
                Some(&*(ptr as *mut DVec3))
            }
        } else {
            None
        }
    }

    fn is_holding(value: &Value) -> bool {
        unsafe {
            let mut result = false;
            ffi::vt_Value_IsHolding_GfVec3d(value.ptr, &mut result);
            result
        }
    }

    fn as_value(member: &Self) -> Value {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::vt_Value_from_GfVec3d(
                *(member as *const DVec3 as *const ffi::gf_Vec3d_t),
                &mut ptr,
            );
            Value { ptr }
        }
    }
}

impl ValueMember for Vec4 {
    fn get(value: &Value) -> Option<&Self> {
        if Self::is_holding(value) {
            unsafe {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_GfVec4f(value.ptr, &mut ptr);
                Some(&*(ptr as *mut Vec4))
            }
        } else {
            None
        }
    }

    fn is_holding(value: &Value) -> bool {
        unsafe {
            let mut result = false;
            ffi::vt_Value_IsHolding_GfVec4f(value.ptr, &mut result);
            result
        }
    }

    fn as_value(member: &Self) -> Value {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::vt_Value_from_GfVec4f(
                *(member as *const Vec4 as *const ffi::gf_Vec4f_t),
                &mut ptr,
            );
            Value { ptr }
        }
    }
}

impl ValueMember for Mat2 {
    fn get(value: &Value) -> Option<&Self> {
        if Self::is_holding(value) {
            unsafe {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_GfMatrix3f(value.ptr, &mut ptr);
                Some(&*(ptr as *mut Mat2))
            }
        } else {
            None
        }
    }

    fn is_holding(value: &Value) -> bool {
        unsafe {
            let mut result = false;
            ffi::vt_Value_IsHolding_GfMatrix2f(value.ptr, &mut result);
            result
        }
    }

    fn as_value(member: &Self) -> Value {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::vt_Value_from_GfMatrix2f(
                *(member as *const Mat2 as *const ffi::gf_Matrix2f_t),
                &mut ptr,
            );
            Value { ptr }
        }
    }
}

impl ValueMember for DMat2 {
    fn get(value: &Value) -> Option<&Self> {
        if Self::is_holding(value) {
            unsafe {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_GfMatrix3d(value.ptr, &mut ptr);
                Some(&*(ptr as *mut DMat2))
            }
        } else {
            None
        }
    }

    fn is_holding(value: &Value) -> bool {
        unsafe {
            let mut result = false;
            ffi::vt_Value_IsHolding_GfMatrix2d(value.ptr, &mut result);
            result
        }
    }

    fn as_value(member: &Self) -> Value {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::vt_Value_from_GfMatrix2d(
                *(member as *const DMat2 as *const ffi::gf_Matrix2d_t),
                &mut ptr,
            );
            Value { ptr }
        }
    }
}

impl ValueMember for Mat3 {
    fn get(value: &Value) -> Option<&Self> {
        if Self::is_holding(value) {
            unsafe {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_GfMatrix3f(value.ptr, &mut ptr);
                Some(&*(ptr as *mut Mat3))
            }
        } else {
            None
        }
    }

    fn is_holding(value: &Value) -> bool {
        unsafe {
            let mut result = false;
            ffi::vt_Value_IsHolding_GfMatrix3f(value.ptr, &mut result);
            result
        }
    }

    fn as_value(member: &Self) -> Value {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::vt_Value_from_GfMatrix3f(
                *(member as *const Mat3 as *const ffi::gf_Matrix3f_t),
                &mut ptr,
            );
            Value { ptr }
        }
    }
}

impl ValueMember for DMat3 {
    fn get(value: &Value) -> Option<&Self> {
        if Self::is_holding(value) {
            unsafe {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_GfMatrix3d(value.ptr, &mut ptr);
                Some(&*(ptr as *mut DMat3))
            }
        } else {
            None
        }
    }

    fn is_holding(value: &Value) -> bool {
        unsafe {
            let mut result = false;
            ffi::vt_Value_IsHolding_GfMatrix3d(value.ptr, &mut result);
            result
        }
    }

    fn as_value(member: &Self) -> Value {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::vt_Value_from_GfMatrix3d(
                *(member as *const DMat3 as *const ffi::gf_Matrix3d_t),
                &mut ptr,
            );
            Value { ptr }
        }
    }
}

impl ValueMember for Mat4 {
    fn get(value: &Value) -> Option<&Self> {
        if Self::is_holding(value) {
            unsafe {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_GfMatrix4f(value.ptr, &mut ptr);
                Some(&*(ptr as *mut Mat4))
            }
        } else {
            None
        }
    }

    fn is_holding(value: &Value) -> bool {
        unsafe {
            let mut result = false;
            ffi::vt_Value_IsHolding_GfMatrix4f(value.ptr, &mut result);
            result
        }
    }

    fn as_value(member: &Self) -> Value {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::vt_Value_from_GfMatrix4f(
                *(member as *const Mat4 as *const ffi::gf_Matrix4f_t),
                &mut ptr,
            );
            Value { ptr }
        }
    }
}

impl ValueMember for DMat4 {
    fn get(value: &Value) -> Option<&Self> {
        if Self::is_holding(value) {
            unsafe {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_GfMatrix4d(value.ptr, &mut ptr);
                Some(&*(ptr as *mut DMat4))
            }
        } else {
            None
        }
    }

    fn is_holding(value: &Value) -> bool {
        unsafe {
            let mut result = false;
            ffi::vt_Value_IsHolding_GfMatrix4d(value.ptr, &mut result);
            result
        }
    }

    fn as_value(member: &Self) -> Value {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::vt_Value_from_GfMatrix4d(
                *(member as *const DMat4 as *const ffi::gf_Matrix4d_t),
                &mut ptr,
            );
            Value { ptr }
        }
    }
}
