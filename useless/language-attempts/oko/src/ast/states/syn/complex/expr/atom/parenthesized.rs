use crate::*;

#[derive(Debug, Clone, Parseable, Visitable)]
pub struct ParenthesizedExpr {
	pub opening_bracket: Seq <"(">,
	pub expr: Box <Expr>,
	pub closing_bracket: Seq <")">
}
