use crate::*;

#[derive(Debug, Clone, Parseable, Visitable)]
pub enum LiteralExpr {
	Int(IntLiteral)
}

#[derive(Debug, Clone, Parseable, Visitable)]
pub struct IntLiteral {
	pub value: Number
}
