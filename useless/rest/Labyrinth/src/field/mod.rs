mod types;
mod impls;
mod digger;

use crate::tile::*;

// Public interface

pub use types::Size;

pub fn init() {
    load();
}

#[inline]
pub fn generate_new_level() {
    unsafe { impls::generate_new_level_impl() }
}

#[inline]
pub fn width() -> Size {
    unsafe { impls::width_impl() }
}

#[inline]
pub fn height() -> Size {
    unsafe { impls::height_impl() }
}

#[inline]
pub fn set(x: Size, y: Size, value: TileType) {
    unsafe { impls::set_impl(x, y, value) }
}

#[inline]
pub fn get(x: Size, y: Size) -> TileType {
    unsafe { impls::get_impl(x, y) }
}

#[inline]
pub fn is_updated() -> bool {
    unsafe { impls::is_updated_impl() }
}

#[inline]
pub fn load() {
    unsafe { impls::load_impl() }
}

#[inline]
pub fn save() {
    unsafe { impls::save_impl() }
}
