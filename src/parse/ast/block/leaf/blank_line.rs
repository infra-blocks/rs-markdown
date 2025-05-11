use crate::parse::{
    segment::blank_line::BlankLineSegment,
    traits::{NomParse, Segment},
};
use nom::{Parser, error::ParseError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlankLine<'a>(BlankLineSegment<'a>);

impl<'a> BlankLine<'a> {
    fn new(segment: BlankLineSegment<'a>) -> Self {
        Self(segment)
    }
}

impl<'a> NomParse<'a> for BlankLine<'a> {
    fn nom_parse<Error: ParseError<&'a str>>(input: &'a str) -> nom::IResult<&'a str, Self, Error> {
        BlankLineSegment::nom_parse.map(Self::new).parse(input)
    }
}

impl<'a> Segment<'a> for BlankLine<'a> {
    fn segment(&self) -> &'a str {
        self.0.segment()
    }
}
