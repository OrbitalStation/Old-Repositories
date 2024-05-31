use core::f32::consts::PI;
use core::ops::*;

macro_rules! declare {
    (@cross $trait:ident $little:ident $is_not:ident $big:ident $op:ident $expr:expr) => {
        auto trait $is_not {}

        impl <T> !$is_not for $op <T> {}

        impl <T: Copy + ~const From <f32> + ~const Mul <Output = T>> const $trait for $big <T> {
            type Output = T;

            ///
            /// Convert radians to degrees.
            /// See <a href="https://en.wikipedia.org/wiki/Radian">Radian</a> and <a href="https://en.wikipedia.org/wiki/Degree_(angle)">Degree</a>.
            ///
            fn $little(self) -> $op <Self::Output> {
                const F: f32 = $expr;
                $op(self.0 * T::from(F))
            }
        }
    };

    (@base $big:ident $little:ident $trait:ident $is_not:ident) => {
        #[derive(Copy, Clone)]
        pub struct $big <T: Copy> (T);

        impl <T: Copy> const From <T> for $big <T> {
            fn from(x: T) -> Self {
                Self(x)
            }
        }

        impl <T: Copy> $big <T> {
            pub const fn get(self) -> T {
                self.0
            }
        }

        #[const_trait]
        pub trait $trait {
            type Output: Copy;

            fn $little(self) -> $big <Self::Output>;
        }

        impl <T: Copy + $is_not> const $trait for T {
            type Output = Self;

            fn $little(self) -> $big <Self::Output> {
                $big::from(self)
            }
        }
    };
}

declare!(@base Degrees degrees ToDegrees IsNotDegrees);
declare!(@base Radians radians ToRadians IsNotRadians);
declare!(@cross ToRadians radians IsNotDegrees Degrees Radians PI / 180.0);
declare!(@cross ToDegrees degrees IsNotRadians Radians Degrees 180.0 / PI);
