use crate::*;

#[derive(Debug, Clone)]
pub struct ScopeItem {
	pub val: TyStmt
}

impl ScopeItem {
	pub const fn from_ty_stmt(val: TyStmt) -> Self {
		Self {
			val
		}
	}
}
