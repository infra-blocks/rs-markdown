use crate::{
    ast::block::BlankLine,
    parse::{
        parser_utils::{at_least_1_space_or_tab, eof, line_ending, space_or_tab},
        traits::ParseLine,
    },
};
use parser::{Map, ParseResult, Parser, one_of, recognize};

pub fn blank_line<'a>(input: &'a str) -> ParseResult<&'a str, BlankLine<'a>> {
    recognize(one_of((
        (space_or_tab, line_ending),
        (at_least_1_space_or_tab, eof),
    )))
    .map(BlankLine::new)
    .parse(input)
}

impl<'a> ParseLine<'a> for BlankLine<'a> {
    fn parse_line(input: &'a str) -> ParseResult<&'a str, Self> {
        blank_line(input)
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
