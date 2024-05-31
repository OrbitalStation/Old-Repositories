use crate::*;

#[spanned]
pub enum ParseEither <A: Parseable, B: Parseable> {
    A(A),
    B(B)
}

#[spanned(enum)]
impl <A: Parseable, B: Parseable> Parseable for ParseEither <A, B> {
    fn __parse(parser: &mut Parser) -> Result <Self, ParseError> {
        let err_a = match parser.parse::<A>() {
			Ok(x) => return Ok(Self::A(x)),
			Err(x) => x
		};
		let err_b = match parser.parse::<B>() {
			Ok(x) => return Ok(Self::B(x)),
			Err(x) => x
		};
		Err(if err_a.span.len() > err_b.span.len() {
			err_a
		} else {
			err_b
		})
    }
}
