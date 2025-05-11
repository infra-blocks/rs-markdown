use crate::parse::{
    traits::{NomParse, Segment},
    utils::{indented_by_less_than_4, is_one_of, line},
};
use nom::{
    Parser,
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::space0,
    combinator::{consumed, eof, recognize},
};

/// A thematic break segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThematicBreakSegment<'a>(&'a str);

impl<'a> ThematicBreakSegment<'a> {
    fn new(segment: &'a str) -> Self {
        Self(segment)
    }

    fn thematic_break<Error: nom::error::ParseError<&'a str>>()
    -> impl Parser<&'a str, Output = &'a str, Error = Error> {
        recognize((
            indented_by_less_than_4,
            alt((Self::asterisks(), Self::hyphens(), Self::underscores())),
            eof,
        ))
    }

    fn asterisks<Error: nom::error::ParseError<&'a str>>()
    -> impl Parser<&'a str, Output = &'a str, Error = Error> {
        recognize((
            tag("*"),
            space0,
            tag("*"),
            space0,
            tag("*"),
            take_while(is_one_of(&['*', ' ', '\t'])),
        ))
    }

    fn hyphens<Error: nom::error::ParseError<&'a str>>()
    -> impl nom::Parser<&'a str, Output = &'a str, Error = Error> {
        recognize((
            tag("-"),
            space0,
            tag("-"),
            space0,
            tag("-"),
            take_while(is_one_of(&['-', ' ', '\t'])),
        ))
    }

    fn underscores<Error: nom::error::ParseError<&'a str>>()
    -> impl Parser<&'a str, Output = &'a str, Error = Error> {
        recognize((
            tag("_"),
            space0,
            tag("_"),
            space0,
            tag("_"),
            take_while(is_one_of(&['_', ' ', '\t'])),
        ))
    }
}

impl<'a> NomParse<'a> for ThematicBreakSegment<'a> {
    fn nom_parse<Error: nom::error::ParseError<&'a str>>(
        input: &'a str,
    ) -> nom::IResult<&'a str, Self, Error> {
        consumed(line.and_then(Self::thematic_break()))
            .map(|(segment, _)| Self::new(segment))
            .parse(input)
    }
}

impl<'a> Segment<'a> for ThematicBreakSegment<'a> {
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

        test_parse_macros!(ThematicBreakSegment);

        failure_case!(should_reject_empty, "");
        failure_case!(should_reject_blank_line, "  \n");
        failure_case!(should_reject_tab_indent, "\t---\n");
        failure_case!(should_reject_four_spaces_indent, "    ---\n");
        failure_case!(should_reject_non_consecutive_tokens, " -_*\n");
        failure_case!(should_reject_with_presence_of_other_characters, "---a\n");

        success_case!(should_work_with_three_underscores, "___\n");
        success_case!(should_work_with_four_underscores, "____\n");
        success_case!(should_work_with_three_hyphens, "---\n");
        success_case!(should_work_with_four_hyphens, "----\n");
        success_case!(should_work_with_three_asterisks, "***\n");
        success_case!(should_work_with_four_asterisks, "****\n");
        success_case!(should_work_with_three_spaces_indent, "   ---\n");
        success_case!(should_work_with_trailing_whitespace, "--- \n");
        success_case!(should_work_with_spaces_interspersed, " - - -\n");
        success_case!(should_work_without_eol, "---");
    }
}
