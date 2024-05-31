// Uses

use crate::tile::*;
use super::{
    types::{Data, Size},
    digger
};
use std::fs::OpenOptions;
use rand::Rng;

// Static

static mut DATA: Data = Data {
    width:  0,
    height: 0,
    tiles: Vec::new(),
    updated: true
};

// Fns

#[inline(never)]
unsafe fn idx(x: Size, y: Size) -> usize {
    x as usize + y as usize * DATA.width as usize
}

pub unsafe fn generate_new_level_impl() {
    let mut rng = rand::thread_rng();
    DATA.width = rng.gen_range(crate::MIN_WIDTH..crate::MAX_WIDTH);
    DATA.height = rng.gen_range(crate::MIN_HEIGHT..crate::MAX_HEIGHT);
    DATA.tiles.resize(DATA.width as usize * DATA.height as usize, WALL);
    DATA.tiles.shrink_to_fit();

    digger::dig();
}

#[inline]
pub unsafe fn width_impl() -> Size {
    DATA.width
}

#[inline]
pub unsafe fn height_impl() -> Size {
    DATA.height
}

pub unsafe fn set_impl(x: Size, y: Size, value: TileType) {
    DATA.updated = true;
    DATA.tiles[idx(x, y)] = value
}

pub unsafe fn get_impl(x: Size, y: Size) -> TileType {
    DATA.tiles[idx(x, y)]
}

pub unsafe fn is_updated_impl() -> bool {
    let u = DATA.updated;
    DATA.updated = false;
    u
}

pub unsafe fn load_impl() {
    if DATA.width != 0 { return }

    let mut file = match OpenOptions::new().read(true).open(crate::FIELD_PATH) {
        Ok(file) => file,
        Err(_) => {
            generate_new_level_impl();
            return
        }
    };

    DATA.width = crate::read_u16(&mut file, crate::FIELD_PATH);
    DATA.height = crate::read_u16(&mut file, crate::FIELD_PATH);
    DATA.tiles.resize(DATA.width as usize * DATA.height as usize, WALL);
    DATA.tiles.shrink_to_fit();

    let mut i = 0;
    while i < DATA.tiles.len() {
        *(&mut DATA.tiles[i] as *mut TileType as *mut u8) = crate::read_u8(&mut file, crate::FIELD_PATH);
        i += 1
    }
}

pub unsafe fn save_impl() {
    let mut file = crate::open_w(crate::FIELD_PATH);
    crate::write_u16(&mut file, crate::FIELD_PATH, DATA.width);
    crate::write_u16(&mut file, crate::FIELD_PATH, DATA.height);
    let mut i = 0;
    while i < DATA.tiles.len() {
        crate::write_u8(&mut file, crate::FIELD_PATH, *(&DATA.tiles[i] as *const TileType as *const u8));
        i += 1
    }
}
