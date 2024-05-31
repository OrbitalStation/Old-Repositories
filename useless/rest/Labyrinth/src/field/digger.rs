use crate::{
    field,
    tile::*,
    plant
};
use super::types::{Digger, Step, Size, DiggerPlant};
use rand::prelude::*;

impl Digger {
    pub const fn new(x: Size, y: Size) -> Self {
        Self { x, y }
    }

    pub fn next(&mut self, rng: &mut ThreadRng, wall_shares: u16, empty_shares: u16) -> Step {
        let shares_of = |ty: TileType| -> u16 {
            match ty {
                WALL => wall_shares,
                _    => empty_shares,
            }
        };

        let x_or = |of: Size, to: i8| {
            if self.x == of { 0 } else { shares_of(field::get((self.x as i16 + to as i16) as Size, self.y)) }
        };

        let y_or = |of: Size, to: i8| {
            if self.y == of { 0 } else { shares_of(field::get(self.x, (self.y as i16 + to as i16) as Size)) }
        };

        let up_share = x_or(0, -1);
        let right_share = y_or(field::height() - 1, 1);
        let down_share = x_or(field::width() - 1, 1);
        let left_share = y_or(0, -1);

        if up_share == 0 || right_share == 0 || down_share == 0 || left_share == 0 { return Step::Stop }

        let v = rng.gen_range(0..(up_share + right_share + down_share + left_share));

        if v < up_share {
            self.y -= 1
        } else if v < (right_share + up_share) {
            self.x += 1
        } else if v < (down_share + right_share + up_share) {
            self.y += 1
        } else {
            self.x -= 1
        }
        Step::Done
    }
}

pub fn dig() {
    let mut digger = Digger::new(0, 0);
    let mut rng = thread_rng();
    let mut plants = [DiggerPlant { remain: 0, wait: 0 }; plant::Type::Count as usize];
    let mut cur_plant = plant::Type::Count;
    let mut i;
    let mut fail: u8;

    for _ in 0..crate::DIGGERS_COUNT {
        digger.x = rng.gen_range(crate::MIN_DIGGER_PLACING_OFFSET..(field::width()  - crate::MAX_DIGGER_PLACING_OFFSET));
        digger.y = rng.gen_range(crate::MIN_DIGGER_PLACING_OFFSET..(field::height() - crate::MAX_DIGGER_PLACING_OFFSET));

        i = 0;
        while i < plant::Type::Count as u8 {
            let ty = plant::Type::from(i);
            plants[i as usize].remain = rng.gen_range(ty.min_by_digger()..ty.max_by_digger());
            plants[i as usize].wait = 0;
            i += 1
        }

        let wall_shares = rng.gen_range(crate::MIN_WALL_SHARES..crate::MAX_WALL_SHARES) as u16;
        let empty_shares = rng.gen_range(crate::MIN_EMPTY_SHARES..crate::MAX_EMPTY_SHARES) as u16;

        while digger.next(&mut rng, wall_shares, empty_shares) != Step::Stop {
            if field::get(digger.x, digger.y) == WALL {
                fail = 0;

                while fail != crate::DIGGER_PLANT_NON_ZERO_FIND_END_IN {
                    cur_plant = plant::Type::from(rng.gen_range(0..(plant::Type::Count as u8)));

                    if plants[cur_plant as usize].remain != 0 && plants[cur_plant as usize].wait == 0 {
                        plants[cur_plant as usize].wait = cur_plant.clustering();
                        break
                    }
                    fail += 1
                }

                i = 0;
                while i < plant::Type::Count as u8 {
                    if plants[i as usize].wait != 0 { plants[i as usize].wait -= 1 }
                    i += 1
                }

                if fail == crate::DIGGER_PLANT_NON_ZERO_FIND_END_IN {
                    field::set(digger.x, digger.y, EMPTY)
                } else {
                    let coefficient = rng.gen_range(0..(crate::DIGGER_EMPTY_COEFFICIENT + cur_plant.digger_coefficient()));
                    field::set(digger.x, digger.y, if coefficient < (cur_plant.digger_coefficient() + crate::DIGGER_EMPTY_COEFFICIENT) {
                        plant::add(digger.x, digger.y, cur_plant);
                        plants[cur_plant as usize].remain -= 1;
                        PLANT
                    } else {
                        EMPTY
                    })
                }
            }
        }
    }
}
