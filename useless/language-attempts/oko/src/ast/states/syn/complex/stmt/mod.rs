modules!(fun ty);

use crate::*;

#[derive(Debug, Clone, Parseable, Visitable)]
pub enum Stmt {
    Ty(TyStmt),
    Fun(FunStmt)
}
