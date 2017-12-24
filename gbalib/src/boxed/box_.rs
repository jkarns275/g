use super::Ptr;
use alloc::alloc;
use core::mem;
use core::ops::{ Deref, DerefMut };

pub struct Box<T: Sized> {
    inner: Ptr<T>
}

impl<T: Sized> Box<T> {
    pub fn new(item: T) -> Self {
        let inner = unsafe { alloc::<T>(1) };
        Box { inner }
    }
}

impl<T: Sized> Deref for Box<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { mem::transmute::<* const T, &T>(self.inner.ptr) }
    }
}

impl<T: Sized> DerefMut for Box<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { mem::transmute::<* mut T, &mut T>(self.inner.ptr_mut) }
    }
}