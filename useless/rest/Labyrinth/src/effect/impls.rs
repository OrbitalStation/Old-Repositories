use super::{
    types::{Effect, PlayerEffectsIterator},
    EffectType
};
use crate::{
    tick::{Tick, Arg},
    creature::CreatureType,
    field::Size
};

static mut EFFECTS: Vec <Effect> = Vec::new();

pub fn effect_check_cb(_: Arg) -> Tick {
    unsafe {
        if EFFECTS.is_empty() { return crate::EFFECT_CHECK_IN }
        let mut i = 0;
        while i < EFFECTS.len() {
            if EFFECTS[i].duration == 0 {
                EFFECTS[i].effect.on_finish(&EFFECTS[i].obj);
                EFFECTS.remove(i);
            } else {
                EFFECTS[i].effect.on_tick(&EFFECTS[i].obj);
                EFFECTS[i].duration -= crate::EFFECT_CHECK_IN;
                i += 1
            }
        }
    }
    crate::EFFECT_CHECK_IN
}

pub unsafe fn add_effect_impl(mut effect: Effect) {
    let mut i = 0;
    while i < EFFECTS.len() {
        if EFFECTS[i] == effect {
            EFFECTS[i].add(effect);
            return
        }
        i += 1
    }
    effect.effect.on_start(&effect.obj);
    EFFECTS.push(effect);
}

// Implementation is here because `next` needs accesses to `EFFECTS`
impl Iterator for PlayerEffectsIterator {
    type Item = EffectType;

    fn next(&mut self) -> Option <Self::Item> {
        unsafe {
            while self.0 < EFFECTS.len() {
                self.0 += 1;
                if EFFECTS[self.0 - 1].obj.r#type() == CreatureType::Player {
                    return Some(EFFECTS[self.0 - 1].effect)
                }
            }
        }
        None
    }
}

pub fn roman(x: Size) -> &'static str {
    match x {
        1 => "I",
        2 => "II",
        3 => "III",
        4 => "IV",
        5 => "V",
        _ => unimplemented!()
    }
}
