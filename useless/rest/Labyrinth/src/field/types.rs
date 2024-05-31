use crate::tile::TileType;

pub type Size = u16;

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Step {
    Done,
    Stop
}

pub struct Digger {
    pub x: Size,
    pub y: Size
}

pub struct Data {
    pub width: Size,
    pub height: Size,
    pub tiles: Vec <TileType>,
    pub updated: bool
}

#[derive(Copy, Clone)]
pub struct DiggerPlant {
    pub remain: u8,
    pub wait: u8
}
