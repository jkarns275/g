use core::ops::{ Deref, DerefMut };
use core::mem;

#[derive(Copy, Clone)]
pub union Ptr<T: Sized> {
    pub ptr: * const T,
    pub ptr_mut: * mut T,
    pub num: u32
}

impl<T: Sized> Ptr<T> {

    pub const fn from_u32(i: u32) -> Self { Ptr { num: i } }

    pub const fn from_ptr(ptr: * const T) -> Self { Ptr { ptr: ptr } }

    pub const fn from_ptr_mut(ptr_mut: * mut T) -> Self { Ptr { ptr_mut } }

    pub unsafe fn from_ref(const_ref: &T) -> Self { Ptr { ptr: mem::transmute::<&T, * const T>(const_ref) } }

    pub unsafe fn from_ref_mut(mut_ref: &mut T) -> Self { Ptr { ptr_mut: mem::transmute::<&mut T, * mut T>(mut_ref) } }

    pub const fn null() -> Self { Ptr { num: 0 } }

    pub const unsafe fn transmute<S: Sized>(self) -> Ptr<S> {
        Ptr::<S>::from_u32(self.num)
    }

    pub const unsafe fn is_null(&self) -> bool { self.num == 0 }

    pub unsafe fn into_ref(self) -> &'static T { mem::transmute(self.ptr) }

    pub unsafe fn into_ref_mut(self) -> &'static mut T { mem::transmute(self.ptr_mut) }
}

impl<T: Sized> Deref for Ptr<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { mem::transmute::<* const T, &T>(self.ptr) }
    }
}

impl<T: Sized> DerefMut for Ptr<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { mem::transmute::<* mut T, &mut T>(self.ptr_mut) }
    }
}