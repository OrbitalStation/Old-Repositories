use crate::*;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BinOpAssociativity {
	Left,
	Right
}

pub type Precedence = u8;

macro_rules! binops {
    ($( $name:ident $assoc:ident $prec:literal $sign:literal)*) => {
		#[derive(Debug, Copy, Clone, Eq, PartialEq)]
		pub enum BinOp {
			$( $name ),*
		}

		impl BinOp {
			pub fn get_prec_assoc(self) -> (Precedence, BinOpAssociativity) {
				$(if self == Self::$name {
					return ($prec, BinOpAssociativity::$assoc)
				})*
				unreachable!()
			}
		}

		impl Parseable for BinOp {
			fn __parse(parser: &mut Parser) -> Result <Self, ParseError> {
				$(if parser.parse::<Seq <$sign>>().is_ok() {
					return Ok(Self::$name)
				})*
				Err(ParseError::new(parser.span(1)))
			}

			fn add_to_span(&mut self, _: usize) {}
		}
	};
}

pub const MIN_PRECEDENCE: Precedence = 1;

binops! {
	Add Left 1 "+"
	Sub Left 1 "-"

	Mul Left 2 "*"
	Div Left 2 "/"
	Mod Left 2 "mod"
}
