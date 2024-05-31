use crate::*;

pub type TypeIndexIterRaw <'b> = (dyn Iterator <Item = (usize, &'b ScopeItem)>);

pub type TypeIndexIter <'a, 'b> = &'a mut TypeIndexIterRaw <'b>;

pub trait TypeIndex : Parseable + std::fmt::Debug + CloneToRaw {
	fn resolute(&self, type_list: TypeIndexIter) -> Option <Type>;

	fn span(&self) -> Span;
}

#[derive(Debug, Visitable)]
pub struct Type {
	#[novisit]
	pub inner: Box <dyn TypeIndex>
}

impl Clone for Type {
	fn clone(&self) -> Self {
		Self {
			inner: unsafe { Box::from_raw(core::mem::transmute(self.inner.clone_to_raw())) }
		}
	}
}

impl Parseable for Type {
	fn __parse(parser: &mut Parser) -> Result <Self, ParseError> where Self: Sized {
		Ok(Self {
			inner: Box::new(SynType::__parse(parser)?)
		})
	}

	fn add_to_span(&mut self, offset: usize) {
		self.inner.add_to_span(offset)
	}
}
