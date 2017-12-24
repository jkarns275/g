#![feature(i128_type, asm, proc_macro, lang_items, core, core_intrinsics, const_fn, untagged_unions, arbitrary_self_types)]
#![no_std]

#![allow(dead_code)]

extern crate gbalib;
use gbalib::alloc::*;
use gbalib::boxed::Ptr;

//#[macro_use]
extern crate gbaimg;
use gbaimg::img_as_palleted_sprite_4bpp;

use core::intrinsics::volatile_store;
use core::*;

fn draw_rect(left: u16, top: u16, right: u16, bottom: u16, color: u16) {
    let mut ix = left as u32;
    let mut address = Ptr::null();
    while ix < right as u32 {
        let mut iy = top as u32;
        while iy < bottom as u32 {
            let index = 2*(240 * iy + ix);
            address = Ptr::<u16>::from_u32(0x06000000 + (index as u32));
            *address = color;
            iy += 1
        }
        ix += 1;
    }
    unsafe { address.num += 4; }
    *address = 0xDEAD;
}

fn draw_img(x: u32, y: u32, width: u32, height: u32, img: &[u16]) {
    let mut address = Ptr::null();
    for ix in 0..width {
        address = Ptr::<u16>::from_u32(0x06000000 + (ix + x + 240 * y) * 2);
        for iy in 0..height {
            *address = img[(ix * height + iy) as usize];
            unsafe { address.num += 2 * 240 ; }
        }
    }
}

pub struct GbaSprite((&'static [u16], &'static [u8]));
static pal: GbaSprite = GbaSprite(img_as_palleted_sprite_4bpp!("wow.png"));

#[no_mangle]
pub extern "C" fn main(_: i32, _: *const *const i8) -> i32 {

    unsafe { alloc_initialize(); }
    let mut t = 0f32;
    let mut a: Ptr<[u8; 2048]> = Ptr::null();

    a = unsafe { alloc::<[u8; 2048]>(1) };
    assert!(true);

    unsafe { free(&mut a); }
    unsafe {
        let mut vmode = Ptr::<u16>::from_u32(0x04000000);
        *vmode = 0x0403;
    }

    loop {
        t += 1.0;
        //draw_rect(0, 0, 240, 80, 0x7FF0);
        //draw_rect(0, 80, 240, 120, 0x70F0);
        //draw_img(0, 0, 32, 32, test);

    }

}

