use crate::{
    field::Size,
    tick::Tick,
    effect::EffectWithDuration
};

pub use crate::creature::Health;

macro_rules! impls {
    ($vis:vis enum $name:ident { $($elem:ident,)+ }) => {
        #[derive(Copy, Clone, Eq, PartialEq)]
        #[repr(u8)]
        $vis enum $name {
            $($elem,)+

            Count
        }

        impl Type {
            impls!{ @method reappearance       -> Tick           where $($elem)+ }
            impls!{ @method eat_to_explore     -> u8             where $($elem)+ }
            impls!{ @method min_by_digger      -> u8             where $($elem)+ }
            impls!{ @method max_by_digger      -> u8             where $($elem)+ }
            impls!{ @method digger_coefficient -> u8             where $($elem)+ }
            impls!{ @method output             -> &'static str   where $($elem)+ }
            impls!{ @method clustering         -> u8             where $($elem)+ }
            impls!{ @method poison_iterator    -> PoisonIterator where $($elem)+ }
        }

        impl core::convert::From <u8> for Type {
            #[inline(always)]
            fn from(x: u8) -> Self {
                unsafe { *(&x as *const u8 as *const Self) }
            }
        }
    };

    (@method $name:ident -> $ret:ty where $($elem:ident)+) => {
        pub fn $name(self) -> $ret {
            match self {
                $(Self::$elem => Impl::<{ Self::$elem }>::$name(),)+
                Self::Count => unimplemented!()
            }
        }
    };
}

pub struct Impl <const T: Type>;

pub trait PlantTrait {
    fn reappearance() -> Tick;

    fn eat_to_explore() -> u8;

    fn min_by_digger() -> u8;

    fn max_by_digger() -> u8;

    fn digger_coefficient() -> u8;

    fn output() -> &'static str;

    fn clustering() -> u8;

    fn poison_iterator() -> PoisonIterator;
}

impls! {
    pub enum Type {
        Blackberry,
        Belladonna,
    }
}

#[derive(Clone)]
pub struct PoisonIterator {
    eff: Vec <EffectWithDuration>,
    cur: u8
}

impl PoisonIterator {
    pub fn empty() -> Self {
        Self {
            eff: Vec::new(),
            cur: 0
        }
    }

    pub fn new(effects: Vec <EffectWithDuration>) -> Self {
        let mut r = Self {
            eff: effects.clone(),
            cur: 0
        };
        r.eff.shrink_to_fit();
        r
    }
}

impl Iterator for PoisonIterator {
    type Item = EffectWithDuration;

    fn next(&mut self) -> Option <Self::Item> {
        if self.cur == self.eff.len() as u8 {
            None
        } else {
            self.cur += 1;
            Some(self.eff[(self.cur - 1) as usize])
        }
    }
}

#[derive(Copy, Clone)]
pub struct Data {
    pub x: Size,
    pub y: Size,
    pub ty: Type
}

impl crate::Findable for Data {
    #[inline(always)]
    fn x(&self) -> Size { self.x }

    #[inline(always)]
    fn y(&self) -> Size { self.y }
}
