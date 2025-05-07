mod equals;
mod hyphens;

use crate::parse::traits::{Parse, Segment};
pub use equals::*;
pub use hyphens::*;
use nom::{IResult, Parser, branch::alt, error::ParseError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SetextHeadingUnderlineSegment<'a> {
    Equals(SetextHeadingEqualsUnderlineSegment<'a>),
    Hyphens(SetextHeadingHyphensUnderlineSegment<'a>),
}

impl SetextHeadingUnderlineSegment<'_> {
    pub fn level(&self) -> u8 {
        match self {
            Self::Equals(segment) => segment.level(),
            Self::Hyphens(segment) => segment.level(),
        }
    }
}

impl<'a> From<SetextHeadingEqualsUnderlineSegment<'a>> for SetextHeadingUnderlineSegment<'a> {
    fn from(segment: SetextHeadingEqualsUnderlineSegment<'a>) -> Self {
        Self::Equals(segment)
    }
}

impl<'a> From<SetextHeadingHyphensUnderlineSegment<'a>> for SetextHeadingUnderlineSegment<'a> {
    fn from(segment: SetextHeadingHyphensUnderlineSegment<'a>) -> Self {
        Self::Hyphens(segment)
    }
}

impl<'a> Parse<'a> for SetextHeadingUnderlineSegment<'a> {
    fn parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error>
    where
        Self: Sized,
    {
        alt((
            SetextHeadingEqualsUnderlineSegment::parse.map(Self::from),
            SetextHeadingHyphensUnderlineSegment::parse.map(Self::from),
        ))
        .parse(input)
    }
}

impl<'a> Segment<'a> for SetextHeadingUnderlineSegment<'a> {
    fn segment(&self) -> &'a str {
        match self {
            Self::Equals(segment) => segment.segment(),
            Self::Hyphens(segment) => segment.segment(),
        }
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
                    assert!(SetextHeadingUnderlineSegment::parse::<Error<&str>>($segment).is_err());
                }
            };
        }

        macro_rules! success_case {
            ($test:ident, $segment:expr, $expected:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(
                        SetextHeadingUnderlineSegment::parse::<Error<&str>>($segment),
                        Ok(("", $expected))
                    );
                }
            };
        }

        failure_case!(should_reject_empty, "");
        failure_case!(should_reject_blank_line, "\n");

        success_case!(
            should_accept_equals,
            "=\n",
            SetextHeadingUnderlineSegment::Equals(
                SetextHeadingEqualsUnderlineSegment::strict_parse("=\n")
            )
        );
        success_case!(
            should_accept_hyphens,
            "-\n",
            SetextHeadingUnderlineSegment::Hyphens(
                SetextHeadingHyphensUnderlineSegment::strict_parse("-\n")
            )
        );
    }
}
