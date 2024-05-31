modules!(body fun_act);

use crate::*;

#[derive(Debug, Clone, Parseable, Visitable)]
pub struct FunStmt {
    pub fun_kw: Seq <"fun">,
    pub name: Ident,
    pub opening_bracket: Seq <"(">,
    pub args: ParseRepeatedly <TypedVal, Surrounded <Seq <",">, Whitespace>, True, 0>,
    pub closing_bracket: Seq <")">,
    pub arrow: Seq <"->">,
    pub ret_ty: Type,
    pub body: FunBody
}
