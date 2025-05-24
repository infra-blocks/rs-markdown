use crate::ast::block::ThematicBreak;
use crate::parse::parsers::{indented_by_less_than_4, line_ending_or_eof, space_or_tab};
use crate::parse::traits::ParseLine;
use parser::{Map, ParseResult, Parser, is_one_of, one_of, recognize, tag, take_while};

// TODO: split ThematicBreak into an enum with three variants:
// - Asterisks
// - Hyphens
// - Underscores
pub fn thematic_break<'a>(input: &'a str) -> ParseResult<&'a str, ThematicBreak<'a>> {
    recognize((
        indented_by_less_than_4,
        one_of((asterisks, hyphens, underscores)),
        line_ending_or_eof,
    ))
    .map(ThematicBreak::new)
    .parse(input)
}

fn asterisks(input: &str) -> ParseResult<&str, &str> {
    recognize((
        tag("*"),
        space_or_tab(),
        tag("*"),
        space_or_tab(),
        tag("*"),
        take_while(is_one_of(&['*', ' ', '\t'])),
    ))
    .parse(input)
}

fn hyphens(input: &str) -> ParseResult<&str, &str> {
    recognize((
        tag("-"),
        space_or_tab(),
        tag("-"),
        space_or_tab(),
        tag("-"),
        take_while(is_one_of(&['-', ' ', '\t'])),
    ))
    .parse(input)
}

fn underscores(input: &str) -> ParseResult<&str, &str> {
    recognize((
        tag("_"),
        space_or_tab(),
        tag("_"),
        space_or_tab(),
        tag("_"),
        take_while(is_one_of(&['_', ' ', '\t'])),
    ))
    .parse(input)
}

impl<'a> ParseLine<'a> for ThematicBreak<'a> {
    fn parse_line(input: &'a str) -> ParseResult<&'a str, Self> {
        thematic_break(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod parse {
        use super::*;
        use crate::parse::test_utils::test_parse_macros;

        test_parse_macros!(ThematicBreak);

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
