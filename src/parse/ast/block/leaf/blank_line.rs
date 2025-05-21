use crate::{ast::block::BlankLine, parse::traits::NomParse};
use nom::{
    Parser,
    branch::alt,
    character::complete::{line_ending, space0, space1},
    combinator::{consumed, eof},
    error::ParseError,
};

impl<'a> NomParse<'a> for BlankLine<'a> {
    fn nom_parse<Error: ParseError<&'a str>>(input: &'a str) -> nom::IResult<&'a str, Self, Error> {
        consumed(alt(((space0, line_ending), (space1, eof))))
            .map(|(segment, _)| Self::new(segment))
            .parse(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod parse {
        use super::*;
        use crate::parse::test_utils::test_parse_macros;

        test_parse_macros!(BlankLine);

        failure_case!(should_reject_empty, "");
        failure_case!(should_reject_line_with_a_char, "    a\n");

        success_case!(should_work_with_one_whitespace, " ");
        success_case!(should_work_with_a_single_newline, "\n");
        success_case!(should_work_with_a_single_tab, "\t");
        success_case!(should_work_with_any_whitespace, " \t\r\n");
    }
}
