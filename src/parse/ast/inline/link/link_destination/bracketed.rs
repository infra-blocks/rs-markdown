use crate::parse::traits::ParseLine;
use crate::{ast::inline::link::BracketedLinkDestination, parse::parsers::escaped_sequence};
use parser::{
    Map, ParseResult, Parser, is_one_of, not, one_of, recognize, repeated, tag, take_while,
};

impl<'a> ParseLine<'a> for BracketedLinkDestination<'a> {
    fn parse_line(input: &'a str) -> ParseResult<&'a str, Self> {
        recognize((
            tag("<"),
            repeated(one_of((
                escaped_sequence,
                take_while(not(is_one_of(&['\n', '\\', '<', '>']))).at_least(1),
            ))),
            tag(">"),
        ))
        .map(Self::new)
        .parse(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod parse {
        use super::*;
        use crate::parse::test_utils::test_parse_macros;

        test_parse_macros!(BracketedLinkDestination);

        failure_case!(should_reject_empty_segment, "");
        failure_case!(should_reject_newline, "\n");
        failure_case!(should_reject_single_opening_bracket, "<");
        failure_case!(should_reject_missing_unescaped_closing_bracket, r"<\>");
        failure_case!(should_reject_duplicate_opening_bracket, r"<<>");

        success_case!(should_work_with_empty_brackets, "<>");
        success_case!(should_work_with_a_parenthesis, "<)>");
        success_case!(should_work_with_several_parentheses, "<()(()))>");
        success_case!(should_work_with_any_characters, "<hellö>");
    }
}
