use crate::*;

#[derive(Debug, Clone, Visitable)]
pub struct Whitespace;

/*

inj Parseable in Whitespace
    fun parse(parser: Parser) -> Tuple[Parser, result[Me, ParseError]]
        ty Single
        inj Parseable in Single
            fun parse(parser: Parser) = ParseEither[Seq[" "], Seq["\t"]]::parse(parser).map[1] { x.map { Me::new } }
        return ParseRepeatedly[Single, IgnoringStaticConsumer, 0]::parse(parse).map { Me::new }

*/
impl Parseable for Whitespace {
    fn __parse(parser: &mut Parser) -> Result <Self, ParseError> {
        #[derive(Debug, Clone, Visitable)]
        struct Single;
        impl Parseable for Single {
            fn __parse(parser: &mut Parser) -> Result <Self, ParseError> {
                parser.parse::<ParseEither <Seq <" ">, Seq <"\t">>>().map(|_| Self)
            }

            fn add_to_span(&mut self, _: usize) {}
        }

        parser.parse::<ParseRepeatedly <Single, NoSep, False, 0>>().map(|_| Self)
    }

    fn add_to_span(&mut self, _: usize) {}
}
