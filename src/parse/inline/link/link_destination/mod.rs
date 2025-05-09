mod bracketed;
mod unbracketed;

use crate::inline::link::{BracketedLinkDestination, LinkDestination, UnbracketedLinkDestination};
use crate::parse::traits::Parse;
use nom::{IResult, Parser, branch::alt, error::ParseError};

impl<'a> Parse<'a> for LinkDestination<'a> {
    fn parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error>
    where
        Self: Sized,
    {
        alt((
            BracketedLinkDestination::parse.map(Self::from),
            UnbracketedLinkDestination::parse.map(Self::from),
        ))
        .parse(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod parse {
        use super::*;
        use crate::parse::traits::StrictParse;
        use nom::error::Error;

        macro_rules! failure_case {
            ($test:ident, $segment:expr) => {
                #[test]
                fn $test() {
                    assert!(LinkDestination::parse::<Error<&str>>($segment).is_err());
                }
            };
        }

        macro_rules! success_case {
            ($test:ident, $segment:expr, $expected:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(
                        LinkDestination::parse::<Error<&str>>($segment),
                        Ok(("", $expected))
                    );
                }
            };
        }

        failure_case!(should_reject_empty_segment, "");
        failure_case!(should_reject_blank_line, "\n");

        success_case!(
            should_work_with_a_bracketed_variant,
            "<bracketed>",
            LinkDestination::Bracketed(BracketedLinkDestination::strict_parse("<bracketed>"))
        );
        success_case!(
            should_work_with_an_unbracketed_variant,
            "unbracketed",
            LinkDestination::Unbracketed(UnbracketedLinkDestination::strict_parse("unbracketed"))
        );
    }
}
