mod bracketed;
mod unbracketed;

use crate::ast::inline::link::{
    BracketedLinkDestination, LinkDestination, UnbracketedLinkDestination,
};
use crate::parse::traits::ParseLine;
use parser::{Map, ParseResult, Parser, one_of};

impl<'a> ParseLine<'a> for LinkDestination<'a> {
    fn parse_line(input: &'a str) -> ParseResult<&'a str, Self> {
        one_of((
            BracketedLinkDestination::parse_line.map(Self::from),
            UnbracketedLinkDestination::parse_line.map(Self::from),
        ))
        .parse(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod parse {
        use super::*;
        use crate::parse::test_utils::{StrictParse, test_parse_macros};

        test_parse_macros!(LinkDestination);

        failure_case!(should_reject_empty_segment, "");
        failure_case!(should_reject_blank_line, "\n");

        success_case!(
            should_work_with_a_bracketed_variant,
            "<bracketed>",
            parsed => LinkDestination::Bracketed(BracketedLinkDestination::strict_parse("<bracketed>"))
        );
        success_case!(
            should_work_with_an_unbracketed_variant,
            "unbracketed",
            parsed => LinkDestination::Unbracketed(UnbracketedLinkDestination::strict_parse("unbracketed"))
        );
    }
}
