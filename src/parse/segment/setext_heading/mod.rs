mod equals;
mod hyphens;

use crate::{Segment, parse::traits::ParseLine};
pub use equals::*;
pub use hyphens::*;
use parser::{Map, Parser, one_of};

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

impl<'a> ParseLine<'a> for SetextHeadingUnderlineSegment<'a> {
    fn parse_line(input: &'a str) -> parser::ParseResult<&'a str, Self> {
        one_of((
            SetextHeadingEqualsUnderlineSegment::parse_line.map(Self::from),
            SetextHeadingHyphensUnderlineSegment::parse_line.map(Self::from),
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
        use crate::parse::test_utils::{StrictParse, test_parse_macros};

        test_parse_macros!(SetextHeadingUnderlineSegment);

        failure_case!(should_reject_empty, "");
        failure_case!(should_reject_blank_line, "\n");

        success_case!(
            should_accept_equals,
            "=\n",
            parsed => SetextHeadingUnderlineSegment::Equals(
                SetextHeadingEqualsUnderlineSegment::strict_parse("=\n")
            )
        );
        success_case!(
            should_accept_hyphens,
            "-\n",
            parsed => SetextHeadingUnderlineSegment::Hyphens(
                SetextHeadingHyphensUnderlineSegment::strict_parse("-\n")
            )
        );
    }
}
