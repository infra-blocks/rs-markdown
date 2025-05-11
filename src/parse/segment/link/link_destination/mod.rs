mod bracketed;
mod unbracketed;

pub use bracketed::*;
pub use unbracketed::*;

use crate::parse::traits::{NomParse, Segment};
use nom::{IResult, Parser, branch::alt, error::ParseError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkDestinationSegment<'a> {
    Bracketed(BracketedLinkDestinationSegment<'a>),
    Unbracketed(UnbracketedLinkDestinationSegment<'a>),
}

impl<'a> From<BracketedLinkDestinationSegment<'a>> for LinkDestinationSegment<'a> {
    fn from(segment: BracketedLinkDestinationSegment<'a>) -> Self {
        LinkDestinationSegment::Bracketed(segment)
    }
}

impl<'a> From<UnbracketedLinkDestinationSegment<'a>> for LinkDestinationSegment<'a> {
    fn from(segment: UnbracketedLinkDestinationSegment<'a>) -> Self {
        LinkDestinationSegment::Unbracketed(segment)
    }
}

impl<'a> NomParse<'a> for LinkDestinationSegment<'a> {
    fn nom_parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error>
    where
        Self: Sized,
    {
        alt((
            BracketedLinkDestinationSegment::nom_parse.map(Self::from),
            UnbracketedLinkDestinationSegment::nom_parse.map(Self::from),
        ))
        .parse(input)
    }
}

impl<'a> Segment<'a> for LinkDestinationSegment<'a> {
    fn segment(&self) -> &'a str {
        match self {
            Self::Bracketed(segment) => segment.segment(),
            Self::Unbracketed(segment) => segment.segment(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod parse {
        use super::*;
        use crate::parse::test_utils::test_parse_macros;

        test_parse_macros!(LinkDestinationSegment);

        failure_case!(should_reject_empty_segment, "");
        failure_case!(should_reject_blank_line, "\n");

        success_case!(
            should_work_with_a_bracketed_variant,
            "<bracketed>",
            parsed => LinkDestinationSegment::Bracketed(BracketedLinkDestinationSegment::new(
                "<bracketed>"
            ))
        );
        success_case!(
            should_work_with_an_unbracketed_variant,
            "unbracketed",
            parsed => LinkDestinationSegment::Unbracketed(UnbracketedLinkDestinationSegment::new(
                "unbracketed"
            ))
        );
    }
}
