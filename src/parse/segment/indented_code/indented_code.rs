use crate::{
    Segment,
    parse::{
        traits::NomParse,
        utils::{indented_by_at_least_4, line, non_whitespace},
    },
};
use nom::{
    IResult, Parser,
    combinator::{recognize, rest},
    error::ParseError,
};

/// An indented code segment.
///
/// An indented code segment is one that starts with 4 spaces or a tab and
/// isn't a blank line segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndentedCodeSegment<'a>(&'a str);

impl<'a> IndentedCodeSegment<'a> {
    fn new(segment: &'a str) -> Self {
        Self(segment)
    }

    fn indented_code<Error: ParseError<&'a str>>(
        input: &'a str,
    ) -> IResult<&'a str, &'a str, Error> {
        recognize((indented_by_at_least_4, non_whitespace, rest)).parse(input)
    }
}

impl<'a> NomParse<'a> for IndentedCodeSegment<'a> {
    fn nom_parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error> {
        recognize(line.and_then(Self::indented_code))
            .map(Self::new)
            .parse(input)
    }
}

impl<'a> Segment<'a> for IndentedCodeSegment<'a> {
    fn segment(&self) -> &'a str {
        self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parse::test_utils::test_parse_macros;

    test_parse_macros!(IndentedCodeSegment);

    failure_case!(should_reject_empty_segment, "");
    failure_case!(should_reject_blank_line, " \n");
    failure_case!(should_reject_3_whitespaces_indent, "   Missing one space\n");

    success_case!(
        should_work_with_4_whitespaces_indent,
        "    This is indented code. Finally.\n"
    );
    success_case!(
        should_work_with_tab_indent,
        "\tThis is indented code. Finally.\n"
    );
    success_case!(
        should_work_with_missing_eol,
        "    This is indented code. Finally."
    );
}
