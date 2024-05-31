use crate::*;

#[derive(Debug, Clone, Parseable, Visitable)]
pub struct TypedVal {
    pub ident: Ident,
    pub colon: Seq <":">,
    pub ty: Type
}
