pub mod bear;

use crate::field::*;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Gender {
    Male,
    Female
}

pub trait Animal {
    //< Animal can be created only by coordinates
    fn new(x: SizeT, y: SizeT) -> Self;

    //< Fn is called every time in a loop
    fn act(&mut self);

    //< Gender of animal
    fn gender(&self) -> Gender;
}

pub trait AnimalMale: Animal {
    #[inline(always)]
    fn gender(&self) -> Gender { Gender::Male }


}
