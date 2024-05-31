use crate::*;

#[derive(Debug, Clone, Parseable, Visitable)]
pub struct ValOrZeroArgFunCallExpr {
	pub value: Ident
}
