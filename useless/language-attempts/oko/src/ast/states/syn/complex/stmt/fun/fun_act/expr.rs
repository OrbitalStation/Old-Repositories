use crate::*;

#[derive(Debug, Clone, Parseable, Visitable)]
pub struct ExprFunAct {
	pub expr: Box <Expr>,
	pub newline: Newline
}
