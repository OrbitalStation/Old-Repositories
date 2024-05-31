use crate::*;

#[spanned]
#[derive(Debug, Clone)]
pub struct ScopeType {
	/// An index of an element in `ScopeASTState::type_list`
	pub index: usize
}

impl Parseable for ScopeType {
	fn __parse(_: &mut Parser) -> Result <Self, ParseError> {
		unimplemented!()
	}

	fn add_to_span(&mut self, _: usize) {
		unimplemented!()
	}
}

impl TypeIndex for ScopeType {
	fn resolute(&self, _: TypeIndexIter)  -> Option <Type> {
		Some(Type { inner: Box::new(self.clone()) })
	}

	fn span(&self) -> Span {
		self.span.val.clone()
	}
}
