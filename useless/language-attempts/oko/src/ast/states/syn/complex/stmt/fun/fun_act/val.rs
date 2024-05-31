use crate::*;

#[derive(Debug, Clone, Parseable, Visitable)]
pub struct ValFunAct {
    pub val_kw: Seq <"val">,
    pub ident: Ident,
    pub eq: Seq <"=">,
    pub init: Box <Expr>,
    pub newline: Newline
}
