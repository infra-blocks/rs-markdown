use crate::parse::{
    segment::blank_line::BlankLineSegment,
    traits::{Parse, Segment},
};
use nom::{Parser, error::ParseError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlankLine<'a>(BlankLineSegment<'a>);

impl<'a> BlankLine<'a> {
    fn new(segment: BlankLineSegment<'a>) -> Self {
        Self(segment)
    }
}

impl<'a> Parse<'a> for BlankLine<'a> {
    fn parse<Error: ParseError<&'a str>>(input: &'a str) -> nom::IResult<&'a str, Self, Error> {
        BlankLineSegment::parse.map(Self::new).parse(input)
    }
}

impl<'a> Segment<'a> for BlankLine<'a> {
    fn segment(&self) -> &'a str {
        self.0.segment()
    }
}
