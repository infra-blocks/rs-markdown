use crate::parse::{
    segment::link::LinkDestinationSegment,
    traits::{NomParse, Segment},
};
use nom::{IResult, Parser, error::ParseError};

/// A link destination segment, as described in the [spec](https://spec.commonmark.org/0.31.2/#link-destination).
pub struct LinkDestination<'a>(LinkDestinationSegment<'a>);

impl<'a> LinkDestination<'a> {
    /// Creates a new [LinkDestination] from a [LinkDestinationSegment].
    fn new(segment: LinkDestinationSegment<'a>) -> Self {
        Self(segment)
    }
}

impl<'a> NomParse<'a> for LinkDestination<'a> {
    fn nom_parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error>
    where
        Self: Sized,
    {
        LinkDestinationSegment::nom_parse
            .map(Self::new)
            .parse(input)
    }
}

impl<'a> Segment<'a> for LinkDestination<'a> {
    fn segment(&self) -> &'a str {
        self.0.segment()
    }
}
