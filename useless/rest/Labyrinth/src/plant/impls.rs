// Uses

use crate::{
    tick::{self, Tick},
    field::{self, Size},
    creature::Creature,
    tile::*
};
use super::types::{Data, Type};
use std::fs::OpenOptions;

// Static

static mut DATA: Vec <Data> = Vec::new();

// Fns

fn regrow_cb(arg: tick::Arg) -> Tick {
    unsafe {
        let me = &mut *(arg as *mut Data);
        let cl = field::get(me.x, me.y);
        if cl == PLAYER {
            return crate::PLANT_CHECK_CELL_FREE_IN
        } else if cl == EMPTY {
            field::set(me.x, me.y, PLANT)
        } else {
            DATA.remove(crate::find_by_coords_index(&DATA, me.x, me.y).unwrap());
        }
    }
    0
}

pub unsafe fn add_impl(x: Size, y: Size, ty: Type) {
    DATA.push(Data { x, y, ty })
}

pub unsafe fn eat_impl(x: Size, y: Size) {
    let plant = crate::find_by_coords(&mut DATA, x, y).expect("There's no plant with such coords!");
    field::set(plant.x, plant.y, EMPTY);
    let creature = Creature::from_coords(plant.x, plant.y);
    for i in plant.ty.poison_iterator() {
        creature.add_effect(i.ty, i.duration)
    }
    tick::add(regrow_cb, plant as *mut Data as tick::Arg, plant.ty.reappearance());
}

pub unsafe fn type_by_coords_impl(x: Size, y: Size) -> Type {
    crate::find_by_coords(&mut DATA, x, y).expect("There's no plant with such coords!").ty
}

pub unsafe fn load_impl() {
    if DATA.len() != 0 { return }

    let mut file = match OpenOptions::new().read(true).open(crate::PLANT_PATH) {
        Ok(file) => file,
        Err(_) => return
    };

    DATA.resize(crate::read_u16(&mut file, crate::PLANT_PATH) as usize, Data {
        x: 0,
        y: 0,
        ty: Type::Count
    });
    DATA.shrink_to_fit();

    let mut i = 0;
    while i < DATA.len() {
        DATA[i].x = crate::read_u16(&mut file, crate::PLANT_PATH);
        DATA[i].y = crate::read_u16(&mut file, crate::PLANT_PATH);
        DATA[i].ty = Type::from(crate::read_u8(&mut file, crate::PLANT_PATH));
        i += 1
    }
}

pub unsafe fn save_impl() {
    let mut file = crate::open_w(crate::PLANT_PATH);
    crate::write_u16(&mut file, crate::PLANT_PATH, DATA.len() as u16);
    let mut i = 0;
    while i < DATA.len() {
        crate::write_u16(&mut file, crate::PLANT_PATH, DATA[i].x);
        crate::write_u16(&mut file, crate::PLANT_PATH, DATA[i].y);
        crate::write_u8(&mut file, crate::PLANT_PATH, DATA[i].ty as u8);
        i += 1
    }
}
