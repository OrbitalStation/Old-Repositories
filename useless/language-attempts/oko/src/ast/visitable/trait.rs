use crate::*;

#[derive(Debug, Clone)]
pub struct VisitProperties {
	/// Process only values within the current scope
	pub scope_limited: bool
}

impl Default for VisitProperties {
	fn default() -> Self {
		Self {
			scope_limited: false
		}
	}
}

impl VisitProperties {
	pub const SCOPE_LIMITED: VisitProperties = VisitProperties {
		scope_limited: true
	};
}

pub trait Visitable {
	/// Recursively goes through the whole composition hierarchy
	/// 	calling `fun` on each value contained within that hierarchy
	/// 	of a type `T`
	fn visit_mut <T> (&mut self, fun: &mut impl VisitCallback <T>, cfg: &VisitProperties);
}

impl <T: Visitable> Visitable for Vec <T> {
	fn visit_mut <U> (&mut self, fun: &mut impl VisitCallback <U>, cfg: &VisitProperties) {
		for x in self.iter_mut() {
			x.visit_mut(fun, cfg)
		}
	}
}
