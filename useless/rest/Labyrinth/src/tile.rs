use bear_lib_terminal::{terminal, geometry};
use crate::{
    plant,
    field::Size
};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct TileType(u8);

impl TileType {
    pub const fn new(x: u8) -> Self {
        Self { 0: x }
    }

    pub fn out(self, p: geometry::Point, x: Size, y: Size) {
        match self {
            PLAYER => terminal::print(p, "[color=red]@"),
            EMPTY  => terminal::print(p, "[color=white]."),
            WALL   => terminal::print(p, "[color=#088715]#"),
            ENEMY  => terminal::print(p, "[color=red]!"),
            PLANT  => terminal::print(p, self.plant_type(x, y).unwrap().output()),
            _ => { }
        }
    }

    pub fn is_oversteppable(self) -> bool {
        match self {
            EMPTY => true,
            PLANT => true,
            _ => false
        }
    }

    pub fn plant_type(self, x: Size, y: Size) -> Option <plant::Type> {
        match self {
            PLANT => Some(plant::type_by_coords(x, y)),
            _ => None
        }
    }

}

pub const PLAYER: TileType = TileType::new(0);
pub const EMPTY:  TileType = TileType::new(1);
pub const WALL:   TileType = TileType::new(2);
pub const ENEMY:  TileType = TileType::new(3);
pub const PLANT:  TileType = TileType::new(4);
