use crate::*;

#[derive(Debug, Clone, Parseable, Visitable)]
pub struct TyStmt {
	pub ty_kw: Seq <"ty">,
	pub name: Ident,
	pub newline: Newline,
	pub fields: Block <TypedVal>
}
