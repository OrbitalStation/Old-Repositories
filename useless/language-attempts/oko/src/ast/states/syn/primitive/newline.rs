use crate::*;

#[derive(Debug, Clone, Visitable)]
pub struct Newline;

impl Parseable for Newline {
    fn __parse(parser: &mut Parser) -> Result <Self, ParseError> {
        #[derive(Debug, Clone, Visitable)]
        struct Single;
        impl Parseable for Single {
            fn __parse(parser: &mut Parser) -> Result <Self, ParseError> {
                parser.whitespace()?;
                parser.parse::<Seq <"\n">>()?;
                Ok(Self)
            }

            fn add_to_span(&mut self, _: usize) {}
        }
        parser.parse::<ParseRepeatedly <Single, NoSep, False, 1>>()?;
        Ok(Self)
    }

    fn add_to_span(&mut self, _: usize) {}
}
