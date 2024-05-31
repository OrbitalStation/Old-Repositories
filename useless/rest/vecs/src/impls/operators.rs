use crate::*;
use core::ops::*;

macro_rules! binop {
    ($( $big:ident $big_assign:ident $low:ident $low_assign:ident $op:tt)*) => {$(
        impl <T: Copy + $big <U>, U: Copy, const N: usize> const $big <vec <U, N>> for vec <T, N> where T::Output: Copy {
            type Output = vec <T::Output, N>;

            #[inline]
            default fn $low(self, rhs: vec <U, N>) -> Self::Output {
                self.fold2(rhs, T::$low)
            }
        }

        impl <T: Copy + $big <U>, U: Copy + NotVec, const N: usize> const $big <U> for vec <T, N> where T::Output: Copy {
            type Output = vec <T::Output, N>;

            #[inline]
            fn $low(self, rhs: U) -> Self::Output {
                self.fold2single(rhs, T::$low)
            }
        }

        impl <T: Copy + $big <U> + ~const From <T::Output>, U: Copy, const N: usize> const $big_assign <vec <U, N>> for vec <T, N> where T::Output: Copy {
            fn $low_assign(&mut self, rhs: vec <U, N>) {
                self.fold2mut(rhs, T::$low)
            }
        }

        impl <T: Copy + $big <U> + ~const From <T::Output>, U: Copy + NotVec, const N: usize> const $big_assign <U> for vec <T, N> where T::Output: Copy {
            fn $low_assign(&mut self, rhs: U) {
                self.fold2single_mut(rhs, T::$low)
            }
        }
    )*};
}

binop! {
    Add AddAssign add add_assign +
    Sub SubAssign sub sub_assign -
    Mul MulAssign mul mul_assign *
    Div DivAssign div div_assign /
    Rem RemAssign rem rem_assign %
    Shr ShrAssign shr shr_assign >>
    Shl ShlAssign shl shl_assign <<
    BitAnd BitAndAssign bitand bitand_assign &
    BitOr BitOrAssign bitor bitor_assign |
    BitXor BitXorAssign bitxor bitxor_assign ^
}

macro_rules! unop {
    ($( $big:ident $low:ident )*) => {$(
        impl <T: Copy + $big, const N: usize> const $big for vec <T, N> where T::Output: Copy {
            type Output = vec <T::Output, N>;

            #[inline]
            fn $low(self) -> Self::Output {
                self.fold1(T::$low)
            }
        }
    )*};
}

unop!(Not not Neg neg);

macro_rules! cmpop {
    ($( $big:ident $( $fn:ident )* ),*) => {$(
        impl <T: Copy, U: Copy, const N: usize> const $big <vec <U, N>> for vec <T, N> where T: ~const PartialEq <U> {
            $(
                #[inline]
                fn $fn(&self, other: &vec <U, N>) -> bool {
                    self.fold_bool(other, T::$fn)
                }
            )*
        }
    )*};
}

cmpop!(PartialEq eq ne);

impl <T: Copy, const N: usize> const Index <usize> for vec <T, N> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl <T: Copy, const N: usize> const IndexMut <usize> for vec <T, N> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl <T: Copy, const N: usize> const Deref for vec <T, N> {
    type Target = [T; N];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl <T: Copy, const N: usize> const DerefMut for vec <T, N> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
