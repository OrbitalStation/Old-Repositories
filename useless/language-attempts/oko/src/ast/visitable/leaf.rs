use crate::*;

#[derive(Debug, Clone, Parseable)]
pub struct Leaf <T> {
	pub val: T
}

impl <T: Copy> Copy for Leaf <T> {}

impl <T> Leaf <T> {
	pub const fn new(val: T) -> Self {
		Self {
			val
		}
	}
}

impl <T> Visitable for Leaf <T> {
	#[inline]
	fn visit_mut <U> (&mut self, fun: &mut impl VisitCallback <U>, _: &VisitProperties) {
		visit_self(&mut self.val, fun)
	}
}
