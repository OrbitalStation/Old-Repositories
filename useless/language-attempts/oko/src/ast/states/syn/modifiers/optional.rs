use crate::*;

#[derive(Debug, Clone)]
pub struct Optional <T: Parseable> (pub Option <T>);

impl <T: Parseable> Parseable for Optional <T> {
    fn __parse(parser: &mut Parser) -> Result <Self, ParseError> {
        Ok(Self(parser.parse::<T>().ok()))
    }

    fn add_to_span(&mut self, offset: usize) {
        if let Some(ref mut x) = self.0 {
            x.add_to_span(offset)
        }
    }
}
/*

inj Parseable in Optional[T]
    fun __parse(parser: Parser) -> (Parser, result[Me, ParseError]) = parser.parse[T].map[1] { Me::new(x.ok) }

*/