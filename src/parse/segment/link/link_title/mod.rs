mod double_quotes;
mod parentheses;
mod single_quotes;

use crate::parse::{input::NomParse, traits::Segment};
pub use double_quotes::*;
use nom::{IResult, Parser, branch::alt, error::ParseError};
pub use parentheses::*;
pub use single_quotes::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkTitleSegment<'a> {
    SingleQuotes(SingleQuotesLinkTitleSegment<'a>),
    DoubleQuotes(DoubleQuotesLinkTitleSegment<'a>),
    Parentheses(ParenthesesLinkTitleSegment<'a>),
}

impl<'a> From<SingleQuotesLinkTitleSegment<'a>> for LinkTitleSegment<'a> {
    fn from(segment: SingleQuotesLinkTitleSegment<'a>) -> Self {
        Self::SingleQuotes(segment)
    }
}

impl<'a> From<DoubleQuotesLinkTitleSegment<'a>> for LinkTitleSegment<'a> {
    fn from(segment: DoubleQuotesLinkTitleSegment<'a>) -> Self {
        Self::DoubleQuotes(segment)
    }
}

impl<'a> From<ParenthesesLinkTitleSegment<'a>> for LinkTitleSegment<'a> {
    fn from(segment: ParenthesesLinkTitleSegment<'a>) -> Self {
        Self::Parentheses(segment)
    }
}

impl<'a> NomParse<'a> for LinkTitleSegment<'a> {
    fn nom_parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error>
    where
        Self: Sized,
    {
        alt((
            SingleQuotesLinkTitleSegment::nom_parse.map(Self::from),
            DoubleQuotesLinkTitleSegment::nom_parse.map(Self::from),
            ParenthesesLinkTitleSegment::nom_parse.map(Self::from),
        ))
        .parse(input)
    }
}

impl<'a> Segment<'a> for LinkTitleSegment<'a> {
    fn segment(&self) -> &'a str {
        match self {
            Self::SingleQuotes(segment) => segment.segment(),
            Self::DoubleQuotes(segment) => segment.segment(),
            Self::Parentheses(segment) => segment.segment(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod parse {
        use super::*;
        use crate::parse::input::strict_parse;
        use nom::error::Error;

        macro_rules! failure_case {
            ($test:ident, $segment:expr) => {
                #[test]
                fn $test() {
                    assert!(LinkTitleSegment::nom_parse::<Error<&str>>($segment).is_err());
                }
            };
        }

        macro_rules! success_case {
            ($test:ident, $segment:expr, $expected:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(
                        LinkTitleSegment::nom_parse::<Error<&str>>($segment),
                        Ok(("", $expected))
                    );
                }
            };
        }

        failure_case!(should_reject_empty_string, "");
        failure_case!(should_reject_blank_line, "\n");

        success_case!(
            should_accept_single_quotes,
            "'hello'",
            LinkTitleSegment::SingleQuotes(strict_parse("'hello'"))
        );
        success_case!(
            should_accept_double_quotes,
            "\"hello\"",
            LinkTitleSegment::DoubleQuotes(strict_parse("\"hello\""))
        );
        success_case!(
            should_accept_parentheses,
            "(hello)",
            LinkTitleSegment::Parentheses(strict_parse("(hello)"))
        );
    }
}
