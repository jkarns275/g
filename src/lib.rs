#![feature(i128_type, asm, lang_items, core, core_intrinsics, const_fn, untagged_unions, arbitrary_self_types)]
#![no_std]

#![allow(dead_code)]

mod lang;
pub use lang::*;

mod gbalib;
use gbalib::alloc::*;
use gbalib::boxed::Ptr;

use core::intrinsics::volatile_store;
use core::*;

#[no_mangle]
pub extern "C" fn main(_: i32, _: *const *const i8) -> i32 {
    unsafe { alloc_initialize(); }

    let mut a: Ptr<[u8; 2048]> = Ptr::null();

    a = unsafe { alloc::<[u8; 2048]>(1) };
    assert!(true);

    unsafe { free(&mut a); }

    let mode_loc = unsafe { core::mem::transmute::<u32, &mut u32>(0x04000000) };
    unsafe { volatile_store(mode_loc, 0x0403); }
    unsafe {
        let address = core::mem::transmute::<u32, * mut u16>(0x06000000);
        volatile_store(address, 0x001F);
    }
    let mut i = 0u16;
    while i < (240*160) {
        unsafe {
            let address = Ptr::<u16>::from_u32(0x06000000 + (2*i) as u32);
            volatile_store(address.ptr_mut, 0xFFFF);
        }
        i += 1;
    }

    loop {}
}

