mod types;
mod impls;

use crate::{
    tick,
    creature::{Health, Creature},
    field::Size
};

macro_rules! impls {
    ($vis:vis enum $name:ident {
        $($elem_name:ident {
            Data = $elem_data:ident where { $($field_name:ident : $field_type:ty,)+ },
            Show Power = $elem_sp:ident,
            Add = $add_lambda:expr,
            Handlers = {
                $(On Start = $on_start:expr,)?
                $(On Finish = $on_finish:expr,)?
                $(On Tick = $on_tick:expr)?
            }
        },)+
    }) => {

        $(#[derive(Copy, Clone, Eq, PartialEq)]
        pub struct $elem_data {
            $(pub $field_name : $field_type,)+
        })+

        $(impl CallableEffect for $elem_data {
            #[inline(always)]
            #[allow(unused_mut)]
            #[allow(unused_variables)]
            fn on_start(&mut self, obj: &Creature) { $(($on_start)(self, obj))? }

            #[inline(always)]
            #[allow(unused_variables)]
            fn on_finish(&self, obj: &Creature) { $(($on_finish)(self, obj))? }

            #[inline(always)]
            #[allow(unused_variables)]
            fn on_tick(&self, obj: &Creature) { $(($on_tick)(self, obj))? }
        })+

        #[derive(Copy, Clone, Eq)]
        #[repr(u8)]
        $vis enum $name {
            $($elem_name($elem_data),)+
        }

        impl EffectType {
            pub fn as_str(&self) -> String {
                format!("[color=orange]{}", match self {
                    $(Self::$elem_name(data) => {
                        format!(concat!(stringify!($elem_name), " {}"), impls::roman(data.$elem_sp as crate::field::Size))
                    },)+
                })
            }

            pub fn add(&mut self, rhs: EffectType) {
                match self {
                    $(Self::$elem_name(ref mut data) => {
                        match rhs {
                            Self::$elem_name(ref rdata) => ($add_lambda)(data, rdata),
                            _ => panic!("Cannot add two different effects!")
                        }
                    },)+
                }
            }

            pub fn as_u8(&self) -> u8 {
                unsafe { *(self as *const Self as *const u8) }
            }

            pub fn on_start(&mut self, obj: &Creature) {
                match self {
                    $(Self::$elem_name(ref mut data) => {
                        data.on_start(obj)
                    },)+
                }
            }

            pub fn on_finish(&self, obj: &Creature) {
                match self {
                    $(Self::$elem_name(ref data) => {
                        data.on_finish(obj)
                    },)+
                }
            }

            pub fn on_tick(&self, obj: &Creature) {
                match self {
                    $(Self::$elem_name(ref data) => {
                        data.on_tick(obj)
                    },)+
                }
            }
        }

        impl PartialEq for EffectType {
            fn eq(&self, other: &Self) -> bool {
                self.as_u8() == other.as_u8()
            }
        }
    };
}

// Public interface

pub use types::*;

impls! {
    pub enum EffectType {
        Poison {
            Data = PoisonEffect where {
                power: Health,
            },
            Show Power = power,
            Add = |data: &mut PoisonEffect, rdata: &PoisonEffect| {
                data.power = data.power.max(rdata.power)
            },
            Handlers = {
                On Tick = |me: &PoisonEffect, obj: &Creature| {
                    obj.decrease_health(me.power)
                }
            }
        },
        Saturation {
            Data = SaturationEffect where {
                power: u8,
            },
            Show Power = power,
            Add = |data: &mut SaturationEffect, rdata: &SaturationEffect| {
                data.power = data.power.max(rdata.power)
            },
            Handlers = {
                On Tick = |me: &SaturationEffect, obj: &Creature| {
                    obj.increase_satiety(me.power)
                }
            }
        },
        Blindness {
            Data = BlindnessEffect where {
                was_vis: Size,
                current: Size,
            },
            Show Power = current,
            Add = |data: &mut BlindnessEffect, rdata: &BlindnessEffect| {
                data.current = data.current.max(rdata.current)
            },
            Handlers = {
                On Start = |me: &mut BlindnessEffect, obj: &Creature| {
                    me.was_vis = obj.get_visibility();
                    obj.set_visibility(me.current)
                },
                On Finish = |me: &BlindnessEffect, obj: &Creature| {
                    obj.set_visibility(me.was_vis)
                },
            }
        },
    }
}

pub fn init() {
    tick::add(impls::effect_check_cb, tick::NULLARG, crate::EFFECT_CHECK_IN)
}

#[inline]
pub fn add_effect(effect: Effect) {
    unsafe { impls::add_effect_impl(effect) }
}

#[inline]
pub fn player_effects() -> PlayerEffectsIterator {
    PlayerEffectsIterator { 0: 0 }
}
