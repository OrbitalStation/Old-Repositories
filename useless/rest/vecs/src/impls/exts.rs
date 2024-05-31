use crate::*;

impl <T: Copy, const N: usize> vec <T, N> {
    #[inline]
    pub const unsafe fn get_unchecked(&self, index: usize) -> &T {
        const_get_unchecked(&self.0, index)
    }

    #[inline]
    #[allow(mutable_transmutes)]
    pub const unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        core::mem::transmute(const_get_unchecked(&self.0, index))
    }
}
