use crate::*;

#[derive(Debug, Clone, Parseable, Visitable)]
pub struct ReturnFunAct {
	pub ret_kw: Seq <"return">,
	pub expr: Box <Expr>,
    pub newline: Newline
}
