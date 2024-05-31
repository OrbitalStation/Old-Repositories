use crate::*;
use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub struct Surrounded <T: Parseable, Sur: Parseable> {
    pub value: T,
    pub _marker: PhantomData <Sur>
}

impl <T: Parseable, Sur: Parseable> Parseable for Surrounded <T, Sur> {
    fn __parse(parser: &mut Parser) -> Result <Self, ParseError> {
        parser.parse::<Sur>()?;
        let value = parser.parse::<T>()?;
        parser.parse::<Sur>()?;
        Ok(Self {
            value,
            _marker: PhantomData
        })
    }

    fn add_to_span(&mut self, offset: usize) {
        self.value.add_to_span(offset)
    }
}
