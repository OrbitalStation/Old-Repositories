use crate::*;
use core::marker::PhantomData;

pub trait Piece <T>: Copy {
    const SIZE: usize;

    unsafe fn construct(self, array: *mut T);
}

impl <T> const Piece <T> for () {
    const SIZE: usize = 0;

    #[inline(always)]
    unsafe fn construct(self, _: *mut T) {}
}

impl <T: ~const From <U> + Copy, U: NotTuple + NotList + Copy> const Piece <T> for U {
    const SIZE: usize = 1;

    unsafe fn construct(self, array: *mut T) {
        *array = T::from(self)
    }
}

impl <T, U: ~const Piece <T>> const Piece <T> for (U,) {
    const SIZE: usize = U::SIZE;

    unsafe fn construct(self, array: *mut T) {
        self.0.construct(array)
    }
}

impl <T, A: ~const Piece <T>, B: ~const Piece <T>> const Piece <T> for (A, B) {
    const SIZE: usize = A::SIZE + B::SIZE;

    unsafe fn construct(self, array: *mut T) {
        self.0.construct(array);
        self.1.construct(offset(array, A::SIZE))
    }
}

impl <T, A: ~const Piece <T>, B: ~const Piece <T>, C: ~const Piece <T>> const Piece <T> for (A, B, C) {
    const SIZE: usize = A::SIZE + B::SIZE + C::SIZE;

    unsafe fn construct(self, array: *mut T) {
        self.0.construct(array);
        self.1.construct(offset(array, A::SIZE));
        self.2.construct(offset(array, A::SIZE + B::SIZE))
    }
}

impl <T, A: ~const Piece <T>, B: ~const Piece <T>, C: ~const Piece <T>, D: ~const Piece <T>> const Piece <T> for (A, B, C, D) {
    const SIZE: usize = A::SIZE + B::SIZE + C::SIZE + D::SIZE;

    unsafe fn construct(self, array: *mut T) {
        self.0.construct(array);
        self.1.construct(offset(array, A::SIZE));
        self.2.construct(offset(array, A::SIZE + B::SIZE));
        self.3.construct(offset(array, A::SIZE + B::SIZE + C::SIZE))
    }
}

impl <T, U: ~const Piece <T> + Copy, const N: usize> const Piece <T> for [U; N] {
    const SIZE: usize = N;

    unsafe fn construct(self, mut array: *mut T) {
        let mut i = 0;
        while i < N {
            self[i].construct(array);
            array = offset(array, 1);
            i += 1
        }
    }
}

impl <T, U: ~const Piece <T> + Copy, const N: usize> const Piece <T> for vec <U, N> {
    const SIZE: usize = N;

    #[inline]
    unsafe fn construct(self, array: *mut T) {
        self.0.construct(array)
    }
}

pub struct VecFn <T, const N: usize> (PhantomData <T>);

impl <Args: ~const Piece <T>, T: ~const Default + Copy, const N: usize> const FnOnce <Args> for VecFn <T, N> {
    type Output = vec <T, N>;

    extern "rust-call" fn call_once(self, args: Args) -> Self::Output {
        assert!(Args::SIZE <= N, "too many args");

        let mut result = vec(uninit());

        let mut ptr = unsafe { result.get_unchecked_mut(Args::SIZE) } as *mut T;
        let mut i = 0;
        while i < (N - Args::SIZE) {
            unsafe {
                *ptr = Default::default();
                ptr = offset(ptr, 1)
            }
            i += 1
        }

        unsafe {
            args.construct(&mut result[0] as *mut T)
        }

        result
    }
}

impl <T: Copy, const N: usize> vec <T, N> {
    #[allow(non_upper_case_globals)]
    pub const new: VecFn <T, N> = VecFn(PhantomData);
}

const unsafe fn offset <T> (array: *mut T, offset: usize) -> *mut T {
    (core::mem::transmute::<_, usize>(array) + core::mem::size_of::<T>() * offset) as *mut T
}
