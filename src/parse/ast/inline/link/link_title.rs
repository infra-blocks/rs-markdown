use crate::parse::{
    segment::link::LinkTitleSegment,
    traits::{NomParse, Segment},
};
use nom::{IResult, Parser, error::ParseError};

/// A link title, as described in the [spec][https://spec.commonmark.org/0.31.2/#link-title].
pub struct LinkTitle<'a>(LinkTitleSegment<'a>);

impl<'a> LinkTitle<'a> {
    /// Creates a new `LinkTitle` from a `LinkTitleSegment`.
    fn new(segment: LinkTitleSegment<'a>) -> Self {
        Self(segment)
    }
}

impl<'a> NomParse<'a> for LinkTitle<'a> {
    fn nom_parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error>
    where
        Self: Sized,
    {
        LinkTitleSegment::nom_parse.map(Self::new).parse(input)
    }
}

impl<'a> Segment<'a> for LinkTitle<'a> {
    fn segment(&self) -> &'a str {
        self.0.segment()
    }
}
