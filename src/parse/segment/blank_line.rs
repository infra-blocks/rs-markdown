use crate::parse::{input::NomParse, traits::Segment};
use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{line_ending, space0, space1},
    combinator::{consumed, eof},
    error::ParseError,
};

/// Represents a blank line segment.
///
/// A blank line contains at least one whitespace character, and only whitespace characters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlankLineSegment<'a>(&'a str);

impl<'a> BlankLineSegment<'a> {
    fn new(segment: &'a str) -> Self {
        Self(segment)
    }
}

impl<'a> NomParse<'a> for BlankLineSegment<'a> {
    fn nom_parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error> {
        consumed(alt(((space0, line_ending), (space1, eof))))
            .map(|(segment, _)| Self::new(segment))
            .parse(input)
    }
}

impl<'a> Segment<'a> for BlankLineSegment<'a> {
    fn segment(&self) -> &'a str {
        self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod parse {
        use super::*;
        use crate::parse::test_utils::test_parse_macros;

        test_parse_macros!(BlankLineSegment);

        failure_case!(should_reject_empty, "");
        failure_case!(should_reject_line_with_a_char, "    a\n");

        success_case!(should_work_with_one_whitespace, " ");
        success_case!(should_work_with_a_single_newline, "\n");
        success_case!(should_work_with_a_single_tab, "\t");
        success_case!(should_work_with_any_whitespace, " \t\r\n");
        success_case!(should_stop_after_newline, "\nhello?", "\n", "hello?");
    }
}
