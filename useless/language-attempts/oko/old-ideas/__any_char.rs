// use crate::*;
//
// #[spanned]
// #[derive(Clone)]
// pub struct AnyChar {}
//
// impl AnyChar {
// 	pub fn get(&self, parser: &Parser) -> char {
// 		parser.code[self.span.start..].chars().next().unwrap()
// 	}
// }
//
// impl Parseable for AnyChar {
// 	fn parse(parser: &mut Parser) -> Result <Self, ParseError> {
// 		let span = parser.span(1);
// 		let next = parser.code[parser.offset.0..].chars().next().ok_or_else(|| ParseError::new(span.clone()))?;
// 		parser.offset.0 += next.len_utf8();
// 		Ok(Self { span })
// 	}
// }
