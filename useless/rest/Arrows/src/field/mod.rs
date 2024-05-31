mod tile;

use sfml::graphics::{RenderWindow, RenderTarget};

pub use tile::*;

pub const SIZE: usize = 50;

pub type Field = [[Tile; SIZE]; SIZE];

pub fn field() -> &'static mut Field {
    unsafe { FIELD_PTR }
}

pub fn opposite_field() -> &'static mut Field {
    unsafe {
        if FIELD_PTR.as_ptr() == (&FIELD1).as_ptr() { &mut FIELD2 } else { &mut FIELD1 }
    }
}

pub fn draw(w: &mut RenderWindow) {
    let mut x = 0;
    let mut y;
    while x < SIZE {
        y = 0;
        while y < SIZE {
            if field()[x][y].ty != TileType::Void {
                field()[x][y].update_sprite(x as f32, y as f32);
                w.draw(Tile::sprite());
            }
            y += 1
        }
        x += 1
    }
}

fn try_rot(x: usize, y: usize, rot: Rotation, f: fn(Rotation, &mut Tile), on: usize) {
    match rot {
        Rotation::Up if y >= on => f(rot, &mut opposite_field()[x][y - on]),
        Rotation::Right if x < SIZE - on => f(rot, &mut opposite_field()[x + on][y]),
        Rotation::Down if y < SIZE - on => f(rot, &mut opposite_field()[x][y + on]),
        Rotation::Left if x >= on => f(rot, &mut opposite_field()[x - on][y]),
        _ => unimplemented!()
    }
}

fn every_rot(x: usize, y: usize, f: fn(Rotation, &mut Tile), on: usize) {
    try_rot(x, y, Rotation::Up, f, on);
    try_rot(x, y, Rotation::Right, f, on);
    try_rot(x, y, Rotation::Down, f, on);
    try_rot(x, y, Rotation::Left, f, on);
}

pub fn execute() {
    let mut x = 0;
    let mut y;

    *opposite_field() = *field();

    while x < SIZE {
        y = 0;
        while y < SIZE {
            match field()[x][y].ty {
                TileType::Generator | TileType::Not | TileType::ActivatedToggle => every_rot(x, y, |rot, tile| tile.try_activate(!rot), 1),
                TileType::ActivatedArrow | TileType::ActivatedRepeater => try_rot(x, y, field()[x][y].rot, |rot, tile| tile.try_activate(!rot), 1),
                TileType::ActivatedLongArrow => try_rot(x, y, field()[x][y].rot, |rot, tile| tile.try_activate(!rot), 2),
                TileType::ActivatedVeryLongArrow => try_rot(x, y, field()[x][y].rot, |rot, tile| tile.try_activate(!rot), 3),
                TileType::And => {
                    let me = &field()[x][y];
                    let mut doit = false;
                    match me.rot {
                        Rotation::Up | Rotation::Down => if x != 0 && field()[x - 1][y].ty.is_activated() && x != SIZE - 1 && field()[x + 1][y].ty.is_activated() { doit = true },
                        Rotation::Right | Rotation::Left => if y != 0 && field()[x][y - 1].ty.is_activated() && y != SIZE - 1 && field()[x][y + 1].ty.is_activated() { doit = true }
                        _ => unimplemented!()
                    }
                    if doit { try_rot(x, y, me.rot, |rot, tile| tile.try_activate(!rot), 1) }
                },
                TileType::ActivatedDoubleAngleArrow => {
                    let mut rot =  field()[x][y].rot;
                    try_rot(x, y, rot, |rot, tile| tile.try_activate(!rot), 1);
                    rot.next();
                    try_rot(x, y, rot, |rot, tile| tile.try_activate(!rot), 1);
                },
                TileType::ActivatedDoubleStraightArrow => {
                    let mut rot =  field()[x][y].rot;
                    rot.next();
                    try_rot(x, y, rot, |rot, tile| tile.try_activate(!rot), 1);
                    try_rot(x, y, !rot, |rot, tile| tile.try_activate(!rot), 1);
                }
                _ => ()
            }

            y += 1
        }
        x += 1
    }

    unsafe { FIELD_PTR = opposite_field() }

    x = 0;
    while x < SIZE {
        y = 0;
        while y < SIZE {
            let tile = &mut field()[x][y];
            if tile.was_activated { tile.ty.activate() } else { tile.ty.deactivate() }
            tile.was_activated = false;
            y += 1
        }
        x += 1
    }
}

static mut FIELD1: Field = [[Tile::VOID; SIZE]; SIZE];
static mut FIELD2: Field = [[Tile::VOID; SIZE]; SIZE];
static mut FIELD_PTR: &'static mut Field = unsafe { &mut FIELD1 };
