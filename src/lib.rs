#![feature(i128_type, asm, proc_macro, lang_items, core, core_intrinsics, const_fn, untagged_unions, arbitrary_self_types)]
#![no_std]

#![allow(dead_code)]

extern crate gbalib;
use gbalib::alloc::*;
use gbalib::boxed::Ptr;
use gbalib::mem::{ memcpy };
use gbalib::collections::*;
use gbalib::graphics::sprites::*;
use gbalib::graphics::*;

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

fn oam_init(obj: Ptr<SpriteAttributes>, mut count: u32) {
    let mut ptr: Ptr<u32> = unsafe { obj.transmute() };
    while count != 0 {
        count -= 1;
        *ptr = AffineMode::Disabled as u32;
        unsafe { ptr.offset(1) }
        *ptr = 0;
        unsafe { ptr.offset(1) }
    }
}

pub struct GbaSprite(&'static [u16], &'static [u8]) ;
static pal: (&'static [u16], &'static [u8]) = img_as_palleted_sprite_4bpp!("wow.png");

const obj_buffer: StaticArr<SpriteAttributes> = StaticArr::new(Ptr::from_u32(0x07000000), 128);
const obj_affine: StaticArr<SpriteAffine> = StaticArr::new(Ptr::from_u32(0x07000000), 32);

type Charblock = [u8; 0x4000];

const tile_mem: StaticArr<Charblock> = StaticArr::new(Ptr::from_u32(0x06000000), 6);
const sprite_pal: StaticArr<u16> = StaticArr::new(Ptr::from_u32(0x05000200), 256);
#[no_mangle]
pub unsafe extern "C" fn main(_: i32, _: *const *const i8) -> i32 {
    let mut config = GraphicsMode::from_u16(0);
    config.sprites_enabled = true;
    config.sprite_storage_mode = SpriteStorageMode::_1D;
    config.set();

    memcpy(Ptr::from_ref(&tile_mem[4][0]), Ptr::from_ptr(pal.1.as_ptr()), 10);
    memcpy(sprite_pal.as_ptr(), Ptr::from_ptr(pal.0.as_ptr()), pal.0.len() as u32);

    let mut sprite = &mut obj_buffer[0];
    sprite.set_dimensions(SpriteDimensions::_64x64);
    sprite.set_color_mode(ColorMode::_4bpp);
    sprite.set_sprite_mode(SpriteMode::Normal);

    loop {}
}
