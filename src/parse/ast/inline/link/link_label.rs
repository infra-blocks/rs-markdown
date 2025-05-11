use crate::parse::{input::NomParse, segment::link::LinkLabelSegment, traits::Segment};
use nom::{IResult, Parser, error::ParseError};

/// A link label, as described in the [spec][https://spec.commonmark.org/0.31.2/#link-label].
pub struct LinkLabel<'a>(LinkLabelSegment<'a>);

impl<'a> LinkLabel<'a> {
    /// Creates a new `LinkLabel` from a `LinkLabelSegment`.
    fn new(segment: LinkLabelSegment<'a>) -> Self {
        Self(segment)
    }
}

impl<'a> NomParse<'a> for LinkLabel<'a> {
    fn nom_parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error>
    where
        Self: Sized,
    {
        LinkLabelSegment::nom_parse.map(Self::new).parse(input)
    }
}

impl<'a> Segment<'a> for LinkLabel<'a> {
    fn segment(&self) -> &'a str {
        self.0.segment()
    }
}
