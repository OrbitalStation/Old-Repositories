use crate::{
    player,
    effect::{self, Effect, EffectType},
    tick::Tick,
    field::Size
};

pub type Health = u8;

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum CreatureType {
    Player
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Creature {
    ty: CreatureType
    // TODO: Make other entities using coordinates(x y), not ptr
}

impl Creature {
    pub fn from_coords(x: Size, y: Size) -> Self {
        if player::is_on(x, y) {
            return Self { ty: CreatureType::Player }
        }
        panic!("There's no one creature on such coords!")
    }

    #[inline(always)]
    pub fn r#type(&self) -> CreatureType {
        self.ty
    }

    pub fn increase_health(&self, health: Health) {
        match self.ty {
            CreatureType::Player => player::increase_health(health)
        }
    }

    pub fn decrease_health(&self, health: Health) {
        match self.ty {
            CreatureType::Player => player::decrease_health(health)
        }
    }

    pub fn increase_satiety(&self, satiety: u8) {
        match self.ty {
            CreatureType::Player => player::increase_satiety(satiety)
        }
    }

    pub fn decrease_satiety(&self, satiety: u8) {
        match self.ty {
            CreatureType::Player => player::decrease_satiety(satiety)
        }
    }

    //noinspection RsSelfConvention
    pub fn set_visibility(&self, new: Size) {
        match self.ty {
            CreatureType::Player => player::set_visibility(new)
        }
    }

    pub fn get_visibility(&self) -> Size {
        match self.ty {
            CreatureType::Player => player::get_visibility()
        }
    }

    pub fn add_effect(&self, effect: EffectType, duration: Tick) {
        effect::add_effect(Effect {
            effect,
            duration,
            obj: *self
        })
    }
}
