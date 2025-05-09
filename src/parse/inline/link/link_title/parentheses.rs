use crate::{
    inline::link::ParenthesesLinkTitle,
    parse::{
        traits::Parse,
        utils::{does_not_contain_blank_line, escaped_sequence},
    },
};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::{complete::is_not, tag},
    combinator::{recognize, verify},
    error::ParseError,
    multi::many0,
};

/*
From the spec:
A link title consists of either:
...
- a sequence of zero or more characters between matching parentheses ((...)), including a ( or ) character only if it is backslash-escaped.
...
Although link titles may span multiple lines, they may not contain a blank line.
*/
impl<'a> Parse<'a> for ParenthesesLinkTitle<'a> {
    fn parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error>
    where
        Self: Sized,
    {
        verify(
            recognize((
                tag("("),
                many0(alt((escaped_sequence, is_not("\\()")))),
                tag(")"),
            )),
            does_not_contain_blank_line,
        )
        .map(Self::new)
        .parse(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod parse {
        use super::*;
        use nom::error::Error;

        macro_rules! failure_case {
            ($test:ident, $segment:expr) => {
                #[test]
                fn $test() {
                    assert!(ParenthesesLinkTitle::parse::<Error<&str>>($segment).is_err());
                }
            };
        }

        macro_rules! success_case {
            ($test:ident, $segment:expr) => {
                success_case!($test, $segment, $segment, "");
            };
            ($test:ident, $segment:expr, $parsed:expr, $remaining:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(
                        ParenthesesLinkTitle::parse::<Error<&str>>($segment),
                        Ok(($remaining, ParenthesesLinkTitle::new($parsed)))
                    );
                }
            };
        }

        failure_case!(should_reject_empty_string, "");
        failure_case!(should_reject_blank_line, "\n");
        failure_case!(should_reject_leading_whitespace, " ()");
        failure_case!(should_reject_missing_closing_parenthesis, "(");
        failure_case!(should_reject_missing_opening_parenthesis, ")");
        failure_case!(
            should_reject_blank_line_within_content,
            "(Hello\nWorld\n\nThis was a blank line!)"
        );

        success_case!(should_accept_empty_content, "()");
        success_case!(should_accept_whitespace_content, "( \t)");
        success_case!(should_accept_text_content, "(Hello)");
        success_case!(should_accept_escaped_parentheses, r"(Hello, \(Bro\))");
        success_case!(should_accept_multline_content, "(Hello\nWorld)");
        success_case!(should_stop_after_closing_parenthesis, "() ", "()", " ");
        success_case!(
            should_stop_after_closing_quote_in_multiline_context,
            "(Hello\n\\(World\\))\n",
            "(Hello\n\\(World\\))",
            "\n"
        );
    }
}
