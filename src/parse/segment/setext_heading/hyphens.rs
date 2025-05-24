use crate::{
    Segment,
    parse::{
        parsers::{indented_by_less_than_4, line_ending_or_eof, space_or_tab},
        traits::ParseLine,
    },
};
use parser::{Map, Parser, equals, recognize, take_while};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetextHeadingHyphensUnderlineSegment<'a>(&'a str);

impl<'a> SetextHeadingHyphensUnderlineSegment<'a> {
    fn new(segment: &'a str) -> Self {
        Self(segment)
    }

    pub fn level(&self) -> u8 {
        2
    }
}

impl<'a> ParseLine<'a> for SetextHeadingHyphensUnderlineSegment<'a> {
    fn parse_line(input: &'a str) -> parser::ParseResult<&'a str, Self> {
        recognize((
            indented_by_less_than_4,
            take_while(equals('-')).at_least(1),
            space_or_tab(),
            line_ending_or_eof,
        ))
        .map(Self::new)
        .parse(input)
    }
}

impl<'a> Segment<'a> for SetextHeadingHyphensUnderlineSegment<'a> {
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

        test_parse_macros!(SetextHeadingHyphensUnderlineSegment);

        failure_case!(should_fail_with_empty, "");
        failure_case!(should_fail_with_blank_line, "\n");
        failure_case!(should_fail_with_4_idents, "    -\n");
        failure_case!(should_fail_with_tab_ident, "\t-\n");
        failure_case!(should_reject_trailing_characters, "-a\n");
        failure_case!(should_fail_for_equals, "=\n");

        success_case!(should_work_with_single_equal, "-\n");
        success_case!(should_work_without_eol, "-");
        success_case!(should_work_with_5_hyphens, "-----\n");
        success_case!(should_work_with_3_idents, "   -\n");
        success_case!(should_work_with_trailing_whitespace, "-  \n");
    }
}
