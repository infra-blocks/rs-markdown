mod double_quotes;
mod parentheses;
mod single_quotes;

use crate::{
    ast::inline::link::{
        DoubleQuotesLinkTitle, LinkTitle, ParenthesesLinkTitle, SingleQuotesLinkTitle,
    },
    parse::{input::Input, traits::Parse},
};
use parser::{Map, ParseResult, Parser, one_of};

impl<'a> Parse<'a> for LinkTitle<'a> {
    fn parse<I: Input<'a>>(input: I) -> ParseResult<I, Self> {
        one_of((
            SingleQuotesLinkTitle::parse.map(Self::SingleQuotes),
            DoubleQuotesLinkTitle::parse.map(Self::DoubleQuotes),
            ParenthesesLinkTitle::parse.map(Self::Parentheses),
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

        test_parse_macros!(LinkTitle);

        failure_case!(should_reject_empty_string, "");
        failure_case!(should_reject_blank_line, "\n");

        success_case!(
            should_accept_single_quotes,
            "'hello'",
            parsed => LinkTitle::SingleQuotes(SingleQuotesLinkTitle::strict_parse("'hello'"))
        );
        success_case!(
            should_accept_double_quotes,
            "\"hello\"",
            parsed => LinkTitle::DoubleQuotes(DoubleQuotesLinkTitle::strict_parse("\"hello\""))
        );
        success_case!(
            should_accept_parentheses,
            "(hello)",
            parsed => LinkTitle::Parentheses(ParenthesesLinkTitle::strict_parse("(hello)"))
        );
    }
}
