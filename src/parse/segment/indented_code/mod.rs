mod continuation;
mod indented_code_or_blank_line;

pub use continuation::*;
pub use indented_code_or_blank_line::*;

use crate::{
    Segment,
    parse::{
        parser::{Map, ParseResult, Parser, recognize, rest},
        parser_utils::{indented_by_at_least_4, is_blank_line},
        traits::ParseLine,
    },
};

pub fn indented_code<'a>(input: &'a str) -> ParseResult<&'a str, IndentedCodeSegment<'a>> {
    if is_blank_line(input) {
        return Err(input);
    }

    recognize((indented_by_at_least_4, rest))
        .map(IndentedCodeSegment::new)
        .parse(input)
}

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
}

impl<'a> ParseLine<'a> for IndentedCodeSegment<'a> {
    fn parse_line(input: &'a str) -> ParseResult<&'a str, Self> {
        indented_code(input)
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
