modules!(atom binop);

use crate::*;

#[spanned]
#[derive(Debug, Clone, Visitable)]
pub enum Expr {
    Atom(ExprAtom),
    BinOp(Box <ExprBinOp>)
}

#[spanned(enum)]
impl Parseable for Expr {
    fn __parse(parser: &mut Parser) -> Result <Self, ParseError> {
        launch_precedence_climbing(parser, MIN_PRECEDENCE)
    }
}
