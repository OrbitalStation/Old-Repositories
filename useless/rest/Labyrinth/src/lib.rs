#![allow(incomplete_features)]
#![feature(stmt_expr_attributes)]
#![feature(const_panic)]
#![feature(array_methods)]
#![feature(const_generics)]

pub mod tile;
pub mod tick;
//pub mod book;
pub mod field;
//pub mod animal;
pub mod plant;
pub mod player;
pub mod effect;
pub mod creature;

use bear_lib_terminal::terminal;
use field::Size;
use tick::Tick;

const MIN_WIDTH: Size = 300; //< Minimal width of field
const MAX_WIDTH: Size = 1000; //< Maximal width of field

const MIN_HEIGHT: Size = 300; //< Minimal height of field
const MAX_HEIGHT: Size = 1000; //< Maximal height of field

const MIN_WALL_SHARES: u8 = 2; //< Minimal shares of walls in process of choosing next tile by digger
const MAX_WALL_SHARES: u8 = 7; //< Maximal shares of walls in process of choosing next tile by digger

const MIN_EMPTY_SHARES: u8 = 1; //< Minimal shares of empty tiles in process of choosing next tile by digger
const MAX_EMPTY_SHARES: u8 = 6; //< Maximal shares of empty tiles in process of choosing next tile by digger

const MIN_DIGGER_PLACING_OFFSET: Size = 5; //< Minimal index of tile can be chosen to place digger
const MAX_DIGGER_PLACING_OFFSET: Size = 5; //< Maximal index of tile can be chosen to place digger (FIELD.width\height - MAX_DIGGER_PLACING_OFFSET)

const PLANT_CHECK_CELL_FREE_IN: Tick = 3; //< How often does plant check that cell it is on is free

const DIGGER_EMPTY_COEFFICIENT: u8 = 8; //< Chance that cell will be empty

const DIGGER_PLANT_NON_ZERO_FIND_END_IN: u8 = 50; //< After N fails, treat as plants are over

const DIGGERS_COUNT: u8 = 6; //< Number of diggers; Cannot be changed because it is most optimal value

const PLAYER_START_VISIBILITY: Size = 10; //< Visibility player starts with
const PLAYER_MAX_VISIBILITY: Size = 10;   //< Max player's visibility

const PLAYER_INTERFACE_START: i32 = 15; //< Start of interface X in terminal
const PLAYER_INTERFACE_HEALTH_ROW: i32 = 1; //< Terminal row where health is located
const PLAYER_INTERFACE_SATIETY_ROW: i32 = 3; //< Terminal row where satiety is located
const PLAYER_EFFECTS_START_ROW: i32 = 5; //< Terminal row where effects started

const PLAYER_HAND_FIND_ON_FAILURE_N_TIMES: u8 = 50; //< N in case that if random-based strategy of finding place for player failed N times, then use hand-based strategy

const PLAYER_MAX_HEALTH: u8 = 100; //< Maximal health value
const PLAYER_MAX_SATIETY: u8 = 100; //< Maximal satiety value

const PLAYER_SATIETY_COUNT: u8 = 10; //< How many actions does player need to waste one satiety point
const PLAYER_STATE_HANDLER_IN: Tick =  3; //< In how many ticks player handles his state

const TERMINAL_WIDTH: u32 = 80;
const TERMINAL_HEIGHT: u32 = 30;

const EFFECT_CHECK_IN: Tick = 1; //< In how many ticks effects are checked

const SAVE_IN: Tick = 100; //< How often is game state saved

///
/// File format:
/// 1st byte <- number of supported plants(for compatibility with old versions)
/// 2nd byte <- number of bytes in data(for compatibility with old versions)
/// data
///
//const BOOK_HERBARIUM_PATH: &'static str = "data/herbarium.dat";

///
/// File format:
/// 1..2 bytes <- width of field
/// 3..4 bytes <- height of field
/// data
///
const FIELD_PATH: &'static str = "data/field.dat";

///
/// File format:
/// 1..2 bytes <- x position
/// 3..4 bytes <- y position
/// 5..6 bytes <- visibility
/// 7th byte   <- health
/// 8th byte   <- satiety
/// 9th byte   <- hunger counter
///
const PLAYER_PATH: &'static str = "data/player.dat";

///
/// File format:
/// 1..2 <- number of plants
/// data
///
const PLANT_PATH: &'static str = "data/plant.dat";

const ON_SAVE_MESSAGE: &'static str = "Saving..";
const ON_LOAD_MESSAGE: &'static str = "Loading..";

fn save_everything(last: bool) {
    if last {
        terminal::clear(None);
        terminal::print_xy(TERMINAL_WIDTH as i32 / 2 - ON_SAVE_MESSAGE.chars().count() as i32 / 2, TERMINAL_HEIGHT as i32 / 2, format!("[color=orange]{}", ON_SAVE_MESSAGE).as_str());
        terminal::refresh()
    } else {
        terminal::print_xy(0, 0, ON_SAVE_MESSAGE);
    }
    field::save();
    player::save();
    plant::save();
    terminal::clear(Some(bear_lib_terminal::geometry::Rect::from_values(0, 0, ON_SAVE_MESSAGE.len() as i32, 1)));
}

pub fn init() {
    terminal::open("Labyrinth", TERMINAL_WIDTH, TERMINAL_HEIGHT);
    terminal::refresh();
    terminal::set(terminal::config::Window::empty().resizeable(true));
    terminal::print_xy(TERMINAL_WIDTH as i32 / 2 - ON_LOAD_MESSAGE.chars().count() as i32 / 2, TERMINAL_HEIGHT as i32 / 2, format!("[color=orange]{}", ON_LOAD_MESSAGE).as_str());
    terminal::refresh();

    field::init();
    player::init();
    effect::init();
    plant::init();

    tick::add(|_| {
        save_everything(false);
        SAVE_IN
    }, tick::NULLARG, SAVE_IN)
}

// Exit(Safe)

pub fn safe_exit() -> ! {
    save_everything(true);
    terminal::close();
    std::process::exit(0);
}

// Find(X & Y)

pub trait Findable {
    fn x(&self) -> Size;
    fn y(&self) -> Size;
}

pub fn find_by_coords <T> (c: &'static mut Vec <T>, x: Size, y: Size) -> Option <&'static mut T> where T: Findable {
    for i in c.iter_mut() {
        if i.x() == x && i.y() == y { return Some(i) }
    }
    None
}

pub fn find_by_coords_index <T> (c: &'static Vec <T>, x: Size, y: Size) -> Option <usize> where T: Findable {
    for (idx, i) in c.iter().enumerate() {
        if i.x() == x && i.y() == y { return Some(idx) }
    }
    None
}

// File (R/W)

pub fn open_w(path: &'static str) -> std::fs::File {
    use std::fs::OpenOptions;

    match OpenOptions::new().truncate(true).write(true).create(true).open(path) {
        Ok(file) => file,
        Err(err) => panic!("Failed to open {} to save data:\n\t{}", path, err)
    }
}

pub fn read_bytes(file: &mut std::fs::File, s: &'static str, buf: &mut [u8]) {
    use std::io::Read;

    if let Err(err) = file.read_exact(buf) {
        panic!("Error while reading {} content:\n\t{}", s, err)
    }
}

pub fn read_u8(file: &mut std::fs::File, s: &'static str) -> u8 {
    let mut buf = [0; 1];
    read_bytes(file, s, buf.as_mut_slice());
    buf[0]
}

pub fn read_u16(file: &mut std::fs::File, s: &'static str) -> u16 {
    let mut buf = [0; 2];
    read_bytes(file, s, buf.as_mut_slice());
    unsafe { *(&buf as *const [u8; 2] as *const u16) }
}

pub fn write_bytes(file: &mut std::fs::File, s: &'static str, buf: &[u8]) {
    use std::io::Write;

    if let Err(err) = file.write_all(buf) {
        panic!("Error while saving data to {}:\n\t{}", s, err)
    }
}

pub fn write_u8(file: &mut std::fs::File, s: &'static str, x: u8) {
    let buf = [x];
    write_bytes(file, s, buf.as_slice());
}

pub fn write_u16(file: &mut std::fs::File, s: &'static str, x: u16) {
    let buf = unsafe { *(&x as *const u16 as *const [u8; 2]) };
    write_bytes(file, s, buf.as_slice());
}
