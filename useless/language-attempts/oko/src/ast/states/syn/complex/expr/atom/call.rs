use crate::*;

#[derive(Debug, Clone, Parseable, Visitable)]
pub struct CallExpr {
    pub fun: Ident,
    pub opening_bracket: Seq <"(">,
    pub args: ParseRepeatedly <Expr, Surrounded <Seq <",">, Whitespace>, True, 0>,
    pub closing_bracket: Seq <")">
}
