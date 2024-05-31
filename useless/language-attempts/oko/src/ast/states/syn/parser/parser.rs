use crate::*;

pub struct Parser <'a> {
	pub code: &'a str,
	pub offset: ByteOffset
}

impl <'a> Parser <'a> {
	pub fn new(code: &'a str) -> Self {
		Self {
			code,
			offset: ByteOffset(0)
		}
	}

	pub fn span(&self, len: usize) -> Span {
		Span::new(self.offset, ByteOffset(self.offset.0 + len))
	}

	/// Just a convenient method
	pub fn whitespace(&mut self) -> Result <Whitespace, ParseError> {
		Whitespace::__parse(self)
	}

	pub fn is_empty(&self) -> bool {
		self.offset.0 == self.code.len()
	}

	pub fn assert_is_empty(&self, err: ParseError) {
		assert!(self.is_empty(), "unexpected error: {:?}", err)
	}

	pub fn parse <T: Parseable> (&mut self) -> Result <T, ParseError> {
		let offset = self.offset;
		T::__parse(self).inspect_err(|_| self.offset = offset)
	}
}
