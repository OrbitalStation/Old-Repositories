mod meta;

pub use meta::*;
use crate::vec;
use core::mem::MaybeUninit;

pub const fn uninit <T> () -> T {
    unsafe { MaybeUninit::uninit().assume_init() }
}

impl <T: Copy, const N: usize> vec <T, N> {
    pub(crate) const fn fold2 <U: Copy, R: Copy, F: ~const Fn(T, U) -> R + Copy> (self, rhs: vec <U, N>, op: F) -> vec <R, N> {
        let mut i = 0;
        let mut result = vec(uninit());
        while i < N {
            unsafe {
                *result.get_unchecked_mut(i) = op(*self.get_unchecked(i), *rhs.get_unchecked(i));
            }
            i += 1
        }
        result
    }

    pub(crate) const fn fold2single <U: Copy, R: Copy, F: ~const Fn(T, U) -> R + Copy> (self, rhs: U, op: F) -> vec <R, N> {
        let mut i = 0;
        let mut result = vec(uninit());
        while i < N {
            unsafe {
                *result.get_unchecked_mut(i) = op(*self.get_unchecked(i), rhs);
            }
            i += 1
        }
        result
    }

    pub(crate) const fn fold1 <R: Copy, F: ~const Fn(T) -> R + Copy> (self, op: F) -> vec <R, N> {
        let mut i = 0;
        let mut result = vec(uninit());
        while i < N {
            unsafe {
                *result.get_unchecked_mut(i) = op(*self.get_unchecked(i))
            }
            i += 1
        }
        result
    }

    pub(crate) const fn fold2mut <U: Copy, R: Copy, F: ~const Fn(T, U) -> R + Copy> (&mut self, rhs: vec <U, N>, op: F) where T: ~const From <R> {
        let mut i = 0;
        while i < N {
            unsafe {
                *self.get_unchecked_mut(i) = T::from(op(*self.get_unchecked(i), *rhs.get_unchecked(i)));
            }
            i += 1
        }
    }

    pub(crate) const fn fold2single_mut <U: Copy, R: Copy, F: ~const Fn(T, U) -> R + Copy> (&mut self, rhs: U, op: F) where T: ~const From <R> {
        let mut i = 0;
        while i < N {
            unsafe {
                *self.get_unchecked_mut(i) = T::from(op(*self.get_unchecked(i), rhs));
            }
            i += 1
        }
    }

    pub(crate) const fn fold_bool <U: Copy, F: ~const Fn(&T, &U) -> bool + Copy> (self, rhs: &vec <U, N>, op: F) -> bool {
        let mut i = 0;
        while i < N {
            unsafe {
                if !op(self.get_unchecked(i), rhs.get_unchecked(i)) {
                    return false
                }
            }
            i += 1
        }
        true
    }
}

pub const unsafe fn const_get_unchecked <T, const N: usize> (array: &[T; N], index: usize) -> &T {
    let array_begin_address: usize = core::mem::transmute(array);
    let elem_address = array_begin_address + index * core::mem::size_of::<T>();
    let elem_address = elem_address as *const T;
    &*elem_address
}
