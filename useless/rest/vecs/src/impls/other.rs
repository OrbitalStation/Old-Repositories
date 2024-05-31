use crate::*;
use core::fmt;

impl <T: fmt::Debug + Copy, const N: usize> fmt::Debug for vec <T, N> {
    fn fmt(&self, f: &mut fmt::Formatter <'_>) -> fmt::Result {
        let name = &core::any::type_name::<Self>()["vecs::".len()..];
        let mut tuple = f.debug_tuple(name);
        let mut i = 0;
        while i < N {
            tuple.field(unsafe { self.get_unchecked(i) });
            i += 1
        }
        tuple.finish()
    }
}

impl <T: Copy, const N: usize> AsRef <[T; N]> for vec <T, N> {
    fn as_ref(&self) -> &[T; N] {
        &self.0
    }
}
