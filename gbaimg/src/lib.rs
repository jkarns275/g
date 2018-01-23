#![feature(proc_macro, rustc_private)]
extern crate proc_macro;
extern crate syn;

extern crate syntax;
use syntax::parse::token;

use proc_macro::{ TokenStream, TokenNode, Term };

extern crate image;
use image::*;

#[macro_use]
extern crate quote;
use quote::*;

use std::fs::File;
use std::mem::transmute;
use std::collections::HashMap;

#[proc_macro]
pub fn img_as_bmp_slice(input: TokenStream) -> TokenStream {

    let img = load_img(input);

    let (width, height) = img.dimensions();

    let mut gba_pixels: Vec<u16> = Vec::with_capacity(width as usize * height as usize);

    for y in 0..height {
        for x in 0..width {
            let rgb = img.get_pixel(x, y).data;
            let converted_red = ((rgb[0] as f32 / 255.0f32) * 31.0f32) as u16;
            let converted_green = ((rgb[1] as f32 / 255.0f32) * 31.0f32) as u16;
            let converted_blue = ((rgb[2] as f32 / 255.0f32) * 31.0f32) as u16;

            let color = (converted_blue << 10) | (converted_green << 5) | converted_red;


            gba_pixels.push(color);
        }
    }

    let mut quoted = quote!{ &#gba_pixels };

    quoted.parse().unwrap()
}

#[proc_macro]
pub fn img_as_palleted_sprite_4bpp(input: TokenStream) -> TokenStream {
    let mut colors = Vec::<u16>::with_capacity(1 << 4);

    let img = load_img(input);
    let (width, height) = img.dimensions();

    if (width * height) & 1 == 1 {
        panic!("image must have an even number of pixels in order to convert it to 4bpp")
    }

    let mut pixels =  vec![0u8; (width * height) as usize >> 1];

    for iy in 0..height {
        for ix in 0..width {
            let rgb = img.get_pixel(ix, iy).data;

            let converted_red = ((rgb[0] as f32 / 255.0f32) * 31.0f32) as u16;
            let converted_green = ((rgb[1] as f32 / 255.0f32) * 31.0f32) as u16;
            let converted_blue = ((rgb[2] as f32 / 255.0f32) * 31.0f32) as u16;

            let color = (converted_blue << 10) | (converted_green << 5) | converted_red;
            let color_index =
                if let Some(color_index) = colors.iter().position(|&item| item == color) {
                    if color_index > 15 {
                        panic!("image contains more than 16 colors: a 4bpp image can only have 16 colors");
                    }
                    color_index
                } else {
                    let color_index = colors.len();
                    if color_index == 16 {
                        panic!("image contains more than 16 colors: a 4bpp image can only have 16 colors");
                    }

                    colors.push(color);
                    color_index
                };

            let nibble_index = (iy * width + ix) as usize;
            let byte_index = nibble_index >> 1;
            if nibble_index & 1 == 0 {
                pixels[byte_index] |= color_index as u8;
            } else {
                pixels[byte_index] |= (color_index << 4) as u8;
            }
        }
    }
    println!("{:?}\n\n{:?}", colors, pixels);
    (quote! { (&#colors, &#pixels) }).parse().unwrap()
}


#[proc_macro]
pub fn img_as_palleted_sprite_8bpp(input: TokenStream) -> TokenStream {
    let mut colors = Vec::<u16>::with_capacity(1 << 8);

    let img = load_img(input);
    let (width, height) = img.dimensions();

    if (width * height) & 1 == 1 {
        panic!("image must have an even number of pixels in order to convert it to 4bpp")
    }

    let mut pixels =  vec![0u8; (width * height) as usize];

    for iy in 0..height {
        for ix in 0..width {
            let rgb = img.get_pixel(ix, iy).data;

            let converted_red = ((rgb[0] as f32 / 255.0f32) * 31.0f32) as u16;
            let converted_green = ((rgb[1] as f32 / 255.0f32) * 31.0f32) as u16;
            let converted_blue = ((rgb[2] as f32 / 255.0f32) * 31.0f32) as u16;

            let color = (converted_blue << 10) | (converted_green << 5) | converted_red;
            let color_index =
                if let Some(color_index) = colors.iter().position(|&item| item == color) {
                    if color_index > 15 {
                        panic!("image contains more than 16 colors: a 4bpp image can only have 16 colors");
                    }
                    color_index
                } else {
                    let color_index = colors.len();
                    if color_index == 16 {
                        panic!("image contains more than 16 colors: a 4bpp image can only have 16 colors");
                    }

                    colors.push(color);
                    color_index
                };

            let byte_index = iy * width + ix;
            pixels[byte_index as usize] = color_index as u8;
        }
    }

    (quote! { (&#colors, &#pixels) }).parse().unwrap()
}

#[proc_macro]
pub fn img_as_palleted_sprite_16bpp(input: TokenStream) -> TokenStream {
    panic!("aa")
}

fn load_img(input: TokenStream) -> image::RgbImage {
    let mut tokens: Vec<_> = input.into_iter().collect();
    if tokens.len() != 1 {
        panic!(format!("Argument should be a single string, but got {} arguments", tokens.len()));
    }

    let file_path = match tokens[0].kind {
        TokenNode::Literal(ref x) => {//proc_macro::Literal(token::Token::Literal(token::Lit::Str_(interned)))) => {
            let trans = unsafe { transmute::<proc_macro::Literal, token::Token>(x.clone()) };
            match trans {
                token::Token::Literal(token::Lit::Str_(s), _) => {
                    s.as_str().to_string()
                },
                token::Token::Literal(token::Lit::StrRaw(s, _), _) => {
                    s.as_str().to_string()
                },
                x @ _ => panic!(format!("Argument should be a string, got {:?}", x))
            }
        },
        ref x @ _ => panic!(format!("Argument should be a string, got {:?}", x))
    };

    let img_res = image::open(&file_path);
    let img = match img_res {
        Ok(i) => i.to_rgb(),
        _ => {
            panic!(format!("could not find image in the specified path '{}'", tokens.len()));
        }
    };

    img
}

