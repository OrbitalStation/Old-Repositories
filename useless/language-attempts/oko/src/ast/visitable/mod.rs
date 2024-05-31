modules!(leaf r#trait);

use crate::*;
use core::any::type_name;

pub trait VisitCallback <T> = FnMut(&mut T);

pub fn visit_self <T, I> (me: &mut I, fun: &mut impl VisitCallback <T>) {
	if type_name::<T>() == type_name::<I>() {
		// SAFETY: T == I
		let reference = unsafe { &mut *(me as *mut I as *mut T) };

		fun(reference);
	}
}
