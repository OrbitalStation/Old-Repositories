use crate::*;
use core::marker::PhantomData;

#[spanned]
#[derive(Clone, Debug, Visitable)]
pub struct StringSeq <F: FnMut(char) -> bool + Default> {
	pub value: Leaf <String>,
	#[novisit]
	pub _marker: PhantomData <F>
}

#[spanned]
impl <F: FnMut(char) -> bool + Default> Parseable for StringSeq <F> {
	fn __parse(parser: &mut Parser) -> Result <Self, ParseError> {
		let mut iter = parser.code[parser.offset.0..].chars().enumerate();
		let mut is_ok = F::default();

		if !is_ok(iter.next().ok_or_else(|| ParseError::new(parser.span(1)))?.1) {
			return Err(ParseError {
				span: parser.span(1)
			})
		}
		let idx = iter.find(|x| !is_ok(x.1)).map(|x| x.0).unwrap_or_else(|| parser.code.len() - parser.offset.0);
		let value = Leaf::new(parser.code[parser.offset.0..parser.offset.0 + idx].to_string());
		let span = Leaf::new(parser.span(idx));
		parser.offset.0 += idx;
		Ok(Self {
			value,
			_marker: PhantomData,
			span
		})
	}
}

#[macro_export]
macro_rules! define_string_seq_alias {
    ($name:ident, $validator:ident($( $body:tt )*) $( $code:tt )*) => {
		pub type $name = StringSeq <$validator>;

		#[derive(Debug, Clone, Copy, Default)]
		pub struct $validator {
			$( $body )*
		}

		impl FnOnce <(char,)> for $validator {
			type Output = bool;

			extern "rust-call" fn call_once(self, _: (char,)) -> bool {
				unimplemented!("don't do that")
			}
		}

		impl FnMut <(char,)> for $validator {
			extern "rust-call" $( $code )*
		}
	};
}
