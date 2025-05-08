use crate::parse::{
    segment::link::LinkLabelSegment,
    traits::{Parse, Segment},
};
use nom::{IResult, Parser, error::ParseError};

/// A link label, as described in the [spec][https://spec.commonmark.org/0.31.2/#link-label].
pub struct LinkLabel<'a>(LinkLabelSegment<'a>);

impl<'a> LinkLabel<'a> {
    /// Creates a new `LinkLabel` from a `LinkLabelSegment`.
    fn new(segment: LinkLabelSegment<'a>) -> Self {
        Self(segment)
    }
}

impl<'a> Parse<'a> for LinkLabel<'a> {
    fn parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error>
    where
        Self: Sized,
    {
        LinkLabelSegment::parse.map(Self::new).parse(input)
    }
}

impl<'a> Segment<'a> for LinkLabel<'a> {
    fn segment(&self) -> &'a str {
        self.0.segment()
    }
}
