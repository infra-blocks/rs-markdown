mod double_quotes;
mod parentheses;
mod single_quotes;

use crate::{
    inline::link::{DoubleQuotesLinkTitle, LinkTitle, ParenthesesLinkTitle, SingleQuotesLinkTitle},
    parse::traits::Parse,
};
use nom::{IResult, Parser, branch::alt, error::ParseError};

impl<'a> Parse<'a> for LinkTitle<'a> {
    fn parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error>
    where
        Self: Sized,
    {
        alt((
            SingleQuotesLinkTitle::parse.map(Self::from),
            DoubleQuotesLinkTitle::parse.map(Self::from),
            ParenthesesLinkTitle::parse.map(Self::from),
        ))
        .parse(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod parse {
        use super::*;
        use crate::parse::{test_utils::test_parse_macros, traits::StrictParse};

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
