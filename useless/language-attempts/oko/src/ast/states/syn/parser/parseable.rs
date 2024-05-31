use crate::*;

pub trait Parseable {
	fn __parse(parser: &mut Parser) -> Result <Self, ParseError> where Self: Sized;

	fn add_to_span(&mut self, offset: usize);
}

impl <T: Parseable> Parseable for Box <T> {
	fn __parse(parser: &mut Parser) -> Result <Self, ParseError> {
		T::__parse(parser).map(|x| Self::new(x))
	}

	fn add_to_span(&mut self, offset: usize) {
		(**self).add_to_span(offset)
	}
}
