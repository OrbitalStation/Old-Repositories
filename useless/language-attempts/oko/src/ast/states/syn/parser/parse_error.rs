use crate::*;

#[derive(Debug, Clone)]
#[constructor]
pub struct ParseError {
	pub span: Span
}
