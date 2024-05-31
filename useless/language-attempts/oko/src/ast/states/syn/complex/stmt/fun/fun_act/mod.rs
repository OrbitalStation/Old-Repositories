modules!(val ret expr);

use crate::*;

#[derive(Debug, Clone, Parseable, Visitable)]
pub enum FunAct {
    Val(ValFunAct),
    Return(ReturnFunAct),
    Stmt(Box <Stmt>),
    Expr(ExprFunAct)
}
