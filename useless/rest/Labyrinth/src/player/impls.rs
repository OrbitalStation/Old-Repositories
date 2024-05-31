// Uses

use crate::{
    tile::*,
    tick::{Arg, Tick},
    field::{self, Size},
    plant,
    effect
};
use super::{
    types::{Data, Flags, VisibilityLevel},
    Direction
};
use std::fs::OpenOptions;
use rand::prelude::*;
use bear_lib_terminal::{
    terminal,
    geometry::Rect
};

// Static

static mut DATA: Data = Data {
    x: 0,
    y: 0,
    visibility: crate::PLAYER_START_VISIBILITY,
    health:     crate::PLAYER_MAX_HEALTH,
    satiety:    crate::PLAYER_MAX_SATIETY,
    hunger_counter: 0,
    flags: Flags::empty()
};

// Fns

pub fn state_handler_cb(_: Arg) -> Tick {
    if unsafe { DATA.health != crate::PLAYER_MAX_HEALTH } && (unsafe { DATA.satiety } >= crate::PLAYER_MAX_SATIETY / 2) {
        super::increase_health(1)
    }
    if unsafe { DATA.satiety } != 0 {
        super::decrease_satiety(1);
    } else {
        super::decrease_health(1)
    }
    crate::PLAYER_STATE_HANDLER_IN
}

pub unsafe fn interact(obj: TileType) {
    if obj == PLANT {
        plant::eat(DATA.x, DATA.y)
    }
}

pub unsafe fn move_impl(dir: Direction, set: bool) {
    let old_x = DATA.x;
    let old_y = DATA.y;
    match dir {
        Direction::Up => {
            if DATA.y == 0 || !field::get(DATA.x, DATA.y - 1).is_oversteppable() { return }
            DATA.y -= 1
        }
        Direction::Right => {
            if DATA.x == field::width() - 1 || !field::get(DATA.x + 1, DATA.y).is_oversteppable() { return }
            DATA.x += 1
        }
        Direction::Down => {
            if DATA.y == field::height() - 1 || !field::get(DATA.x, DATA.y + 1).is_oversteppable() { return }
            DATA.y += 1
        }
        Direction::Left => {
            if DATA.x == 0 || !field::get(DATA.x - 1, DATA.y).is_oversteppable() { return }
            DATA.x -= 1
        }
    }
    if set {
        field::set(old_x, old_y, EMPTY);
        interact(field::get(DATA.x, DATA.y));
        field::set(DATA.x, DATA.y, PLAYER);
        DATA.hunger_counter += 1
    }
}

pub unsafe fn generate_position_impl() {
    let mut rng = thread_rng();

    for _ in 0..crate::PLAYER_HAND_FIND_ON_FAILURE_N_TIMES {
        let x = rng.gen_range(0..(field::width()));
        let y = rng.gen_range(0..(field::height()));
        if field::get(x, y) == EMPTY {
            DATA.x = x;
            DATA.y = y;
            field::set(DATA.x, DATA.y, PLAYER);
            return
        }
    }

    // If we are here, random-based strategy failed
    for x in 0..(field::width()) {
        for y in 0..(field::height()) {
            if field::get(x, y) != WALL {
                DATA.x = x;
                DATA.y = y;
                field::set(DATA.x, DATA.y, PLAYER);
                return
            }
        }
    }

    // if we are here, there's no one free tile on FIELD
    panic!("There's no free tiles to put player on!")
}

pub unsafe fn show_visible_area_impl() {
    if is_visibility_updated_impl() {
        DATA.flags.remove(Flags::VISIBILITY_CHANGED);
        terminal::clear(Some(Rect::from_values(crate::TERMINAL_WIDTH as i32 / 2 - crate::PLAYER_MAX_VISIBILITY as i32 , crate::TERMINAL_HEIGHT as i32 / 2 - crate::PLAYER_MAX_VISIBILITY as i32, crate::PLAYER_MAX_VISIBILITY as i32 * 2 + 1, crate::PLAYER_MAX_VISIBILITY as i32 * 2 + 1)));
    }
    let biggest_layer = 2 * DATA.visibility + 1;
    let mut reverse = false;
    for level in VisibilityLevel::new(DATA.visibility, 0, biggest_layer) {
        let empty = DATA.visibility - level;
        for i in empty..(biggest_layer - empty) {
            let x = DATA.x as i16 + i as i16 - DATA.visibility as i16;
            let y = if reverse { DATA.y as i16 + DATA.visibility as i16 - level as i16 } else { DATA.y as i16 - DATA.visibility as i16 + level as i16 };
            let p = bear_lib_terminal::geometry::Point::new((crate::TERMINAL_WIDTH as i32 / 2) - DATA.visibility as i32 + i as i32, (crate::TERMINAL_HEIGHT as i32 / 2) - DATA.visibility as i32 + if reverse { biggest_layer - level - 1 } else { level } as i32);
            if x < 0 || x >= field::width() as i16 || y < 0 || y >= field::height() as i16 {
                terminal::print(p, "[color=yellow]*");
            } else {
                field::get(x as Size, y as Size).out(p, x as Size, y as Size)
            }
        }
        if level == DATA.visibility - 1 && !reverse {
            reverse = true;
        }
    }
}

pub unsafe fn check_impl() {
    if DATA.hunger_counter >= crate::PLAYER_SATIETY_COUNT {
        if DATA.satiety != 0 { DATA.satiety -= 1 }
        DATA.hunger_counter = 0
    }
}

pub unsafe fn show_interface_impl() {
    terminal::clear(Some(bear_lib_terminal::geometry::Rect::from_point_values(crate::TERMINAL_WIDTH as i32 - crate::PLAYER_INTERFACE_START, 0,crate::TERMINAL_WIDTH as i32, crate::TERMINAL_HEIGHT as i32)));
    terminal::print_xy(crate::TERMINAL_WIDTH as i32 - crate::PLAYER_INTERFACE_START, crate::PLAYER_INTERFACE_HEALTH_ROW, format!("[color=orange]Health:  [color={}]{}",
        if DATA.flags.contains(Flags::SHOW_HEALTH_PLUS) { "green" }
        else if DATA.flags.contains(Flags::SHOW_HEALTH_MINUS) { "red" }
        else { "orange" }
    , DATA.health).as_str());
    terminal::print_xy(crate::TERMINAL_WIDTH as i32 - crate::PLAYER_INTERFACE_START, crate::PLAYER_INTERFACE_SATIETY_ROW, format!("[color=orange]Satiety: [color={}]{}",
        if DATA.flags.contains(Flags::SHOW_SATIETY_PLUS) { "green" }
        else if DATA.flags.contains(Flags::SHOW_SATIETY_MINUS) { "red" }
        else { "orange" }
    , DATA.satiety).as_str());
    let mut row = crate::PLAYER_EFFECTS_START_ROW;
    for i in effect::player_effects() {
        terminal::print_xy(crate::TERMINAL_WIDTH as i32 - crate::PLAYER_INTERFACE_START, row, i.as_str().as_str());
        row += 2
    }
}

pub unsafe fn change_satiety(x: u8, damage: bool) {
    let old = DATA.satiety;
    if damage {
        if x > DATA.satiety { DATA.satiety = 0 }
        else { DATA.satiety -= x }
    } else {
        if DATA.satiety + x < crate::PLAYER_MAX_SATIETY { DATA.satiety += x }
        else { DATA.satiety = crate::PLAYER_MAX_SATIETY }
    }
    if DATA.satiety != old {
        DATA.flags.remove(if DATA.satiety > old { Flags::SHOW_SATIETY_MINUS } else { Flags::SHOW_SATIETY_PLUS  });
        DATA.flags.insert(if DATA.satiety > old { Flags::SHOW_SATIETY_PLUS  } else { Flags::SHOW_SATIETY_MINUS })
    }
}

pub unsafe fn change_health(x: u8, damage: bool) {
    let old = DATA.health;
    if damage {
        if x > DATA.health { DATA.health = 0 }
        else { DATA.health -= x }
    } else {
        if DATA.health + x < crate::PLAYER_MAX_HEALTH { DATA.health += x }
        else { DATA.health = crate::PLAYER_MAX_HEALTH }
    }
    if DATA.health != old {
        DATA.flags.remove(if DATA.health > old { Flags::SHOW_HEALTH_MINUS } else { Flags::SHOW_HEALTH_PLUS  });
        DATA.flags.insert(if DATA.health > old { Flags::SHOW_HEALTH_PLUS  } else { Flags::SHOW_HEALTH_MINUS });
    }
}

#[inline]
pub unsafe fn absorb_hunger_counter_impl() {
    DATA.hunger_counter = 0
}

pub unsafe fn is_on_impl(x: Size, y: Size) -> bool {
    DATA.x == x && DATA.y == y
}

pub unsafe fn set_visibility_impl(new: Size) {
    DATA.flags.insert(Flags::VISIBILITY_CHANGED);
    DATA.visibility = new
}

#[inline]
pub unsafe fn get_visibility_impl() -> Size {
    DATA.visibility
}

#[inline]
pub unsafe fn is_visibility_updated_impl() -> bool {
    DATA.flags.contains(Flags::VISIBILITY_CHANGED)
}

pub unsafe fn load_impl() {
    if DATA.x != 0 { return }

    let mut file = match OpenOptions::new().read(true).open(crate::PLAYER_PATH) {
        Ok(file) => file,
        Err(_) => {
            generate_position_impl();
            return
        }
    };

    DATA.x = crate::read_u16(&mut file, crate::PLAYER_PATH);
    DATA.y = crate::read_u16(&mut file, crate::PLAYER_PATH);
    DATA.visibility = crate::read_u16(&mut file, crate::PLAYER_PATH);
    DATA.health = crate::read_u8(&mut file, crate::PLAYER_PATH);
    DATA.satiety = crate::read_u8(&mut file, crate::PLAYER_PATH);
    DATA.hunger_counter = crate::read_u8(&mut file, crate::PLAYER_PATH);
}

pub unsafe fn save_impl() {
    let mut file = crate::open_w(crate::PLAYER_PATH);
    crate::write_u16(&mut file, crate::PLAYER_PATH, DATA.x);
    crate::write_u16(&mut file, crate::PLAYER_PATH, DATA.y);
    crate::write_u16(&mut file, crate::PLAYER_PATH, DATA.visibility);
    crate::write_u8(&mut file, crate::PLAYER_PATH, DATA.health);
    crate::write_u8(&mut file, crate::PLAYER_PATH, DATA.satiety);
    crate::write_u8(&mut file, crate::PLAYER_PATH, DATA.hunger_counter);
}
