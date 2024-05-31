modules!(call val_or_zero_arg_fun_call parenthesized literal);

use crate::*;

#[derive(Debug, Clone, Parseable, Visitable)]
pub enum ExprAtom {
    Parenthesized(ParenthesizedExpr),
    Call(CallExpr),
    ValOrZeroArgFunCall(ValOrZeroArgFunCallExpr),
    Literal(LiteralExpr)
}
