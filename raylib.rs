#![allow(unused)]
use std::ffi::{c_void, CString};
use std::os::raw::c_char;

#[repr(C)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[repr(C)]
#[derive(Clone)]
pub struct Texture {
    pub id: u32,
    pub width: i32,
    pub height: i32,
    pub mipmaps: i32,
    pub format: i32,
}

type Texture2D = Texture;

#[derive(Debug)]
#[repr(C)]
pub struct Image {
    pub data: *mut c_void,
    pub width: i32,
    pub height: i32,
    pub mipmaps: i32,
    pub format: i32,
}

#[link(name = "raylib", kind = "dylib")]
unsafe extern "C" {
    #[link_name = "InitWindow"]
    fn rl_init_window(width: i32, height: i32, title: *const c_char);

    #[link_name = "WindowShouldClose"]
    fn rl_window_should_close() -> bool;

    #[link_name = "CloseWindow"]
    fn rl_close_window();

    #[link_name = "SetTargetFPS"]
    fn rl_set_target_fps(fps: i32);

    #[link_name = "BeginDrawing"]
    fn rl_begin_drawing();

    #[link_name = "EndDrawing"]
    fn rl_end_drawing();

    #[link_name = "ClearBackground"]
    fn rl_clear_background(color: Color);

    #[link_name = "GetColor"]
    fn rl_get_color(num: u32) -> Color;

    #[link_name = "LoadTextureFromImage"]
    fn rl_load_texture_from_image(image: Image) -> Texture2D;

    #[link_name = "DrawTexture"]
    fn rl_draw_texture(texture: Texture2D, pos_x: i32, pos_y: i32, tint: Color);
}

pub fn init_window(width: i32, height: i32, title: &str) {
    unsafe {
        rl_init_window(width, height, CString::new(title).unwrap().as_ptr());
    }
}

pub fn close_window() {
    unsafe {
        rl_close_window();
    }
}

pub fn end_drawing() {
    unsafe {
        rl_end_drawing();
    }
}

pub fn clear_background(color: Color) {
    unsafe {
        rl_clear_background(color);
    }
}

pub fn begin_drawing() {
    unsafe {
        rl_begin_drawing();
    }
}

pub fn window_should_close() -> bool {
    unsafe { rl_window_should_close() }
}

pub fn get_color(num: u32) -> Color {
    unsafe { rl_get_color(num) }
}

pub fn set_target_fps(fps: i32) {
    unsafe {
        rl_set_target_fps(fps);
    }
}

pub fn load_texture_from_image(image: Image) -> Texture2D {
    unsafe { rl_load_texture_from_image(image) }
}

pub fn draw_texture(texture: Texture2D, pos_x: i32, pos_y: i32, tint: Color) {
    unsafe { rl_draw_texture(texture, pos_x, pos_y, tint) }
}
