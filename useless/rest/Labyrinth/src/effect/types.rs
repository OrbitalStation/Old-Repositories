use crate::{
    creature::Creature,
    tick::Tick
};
use super::EffectType;

// Trait

pub trait CallableEffect {
    fn on_start(&mut self, _: &Creature) { }

    fn on_finish(&self, _: &Creature) { }

    fn on_tick(&self, _: &Creature) { }
}

// Player iterator
pub struct PlayerEffectsIterator(pub(crate) usize);

// Effect

#[derive(Copy, Clone)]
pub struct EffectWithDuration {
    pub ty: EffectType,
    pub duration: Tick
}

impl EffectWithDuration {
    pub fn new(ty: EffectType, duration: Tick) -> Self {
        Self { ty, duration }
    }
}

#[derive(Copy, Clone, Eq)]
pub struct Effect {
    pub effect: EffectType,
    pub duration: Tick,
    pub obj: Creature
}

impl Effect {
    pub fn add(&mut self, rhs: Effect) {
        if *self != rhs { panic!("Cannot add effects of different objs!") }
        self.effect.add(rhs.effect);
        self.duration += rhs.duration
    }
}

impl PartialEq for Effect {
    fn eq(&self, other: &Self) -> bool {
        self.effect == other.effect && self.obj == other.obj
    }
}
