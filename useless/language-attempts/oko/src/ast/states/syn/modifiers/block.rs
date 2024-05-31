use std::fmt::Debug;
use crate::*;

#[derive(Debug, Clone, Visitable)]
pub struct Block <T> (pub Vec <T>);

impl <T: Parseable + Debug + Clone + Visitable> Parseable for Block <T> {
    fn __parse(parser: &mut Parser) -> Result <Self, ParseError> {
        let global_offset = parser.offset.0;
        let nested_code = extract_nested_code(parser);
        let mut nested_parser = Parser::new(&nested_code);
        let result = nested_parser.parse::<ParseRepeatedly <T, NoSep, True, 0>>()?;
        nested_parser.assert_is_empty(*result.last_error);
        let mut result = Self(result.vec);
        for (x, relative_offset) in result.0.iter_mut().zip(1..) {
            // `relative_offset` makes up for each lost '\t'
            x.add_to_span(global_offset + relative_offset)
        }
        Ok(result)
    }

    fn add_to_span(&mut self, offset: usize) {
        for x in &mut self.0 {
            x.add_to_span(offset)
        }
    }
}

fn extract_nested_code(parser: &mut Parser) -> String {
    let mut code = String::new();
    let mut tab_expected = true;
    let mut tab_ruined_by_space = false;
    let mut iter = parser.code[parser.offset.0..].chars().enumerate();
    while let Some((passed, ch)) = iter.next() {
        if tab_expected {
            if ch == '\t' {
                if !tab_ruined_by_space {
                    tab_expected = false
                }
            } else if ch == '\n' {
                tab_ruined_by_space = false
            } else if ch == ' ' {
                tab_ruined_by_space = true
            } else {
                parser.offset.0 += passed;
                return code
            }
            continue
        }
        if ch == '\n' {
            tab_expected = true;
            tab_ruined_by_space = false
        }
        code.push(ch)
    }
    parser.offset.0 = parser.code.len();
    code
}
