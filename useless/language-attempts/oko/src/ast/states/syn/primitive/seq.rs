use std::fmt::{Debug, Formatter};
use crate::*;

#[derive(Clone, Visitable)]
pub struct Seq <const SEQ: &'static str>;

impl <const SEQ: &'static str> Debug for Seq <SEQ> {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		f.write_fmt(format_args!("Seq <\"{SEQ}\">"))
	}
}

impl <const SEQ: &'static str> Parseable for Seq <SEQ> {
	fn __parse(parser: &mut Parser) -> Result <Self, ParseError> {
		let span = parser.span(SEQ.len());
		if !parser.code[parser.offset.0..].starts_with(SEQ) {
			return Err(ParseError::new(span))
		}
		parser.offset.0 += SEQ.len();
		Ok(Seq)
	}

	fn add_to_span(&mut self, _: usize) {}
}
