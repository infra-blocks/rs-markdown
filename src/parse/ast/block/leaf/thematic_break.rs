use crate::parse::{
    segment::thematic_break::ThematicBreakSegment,
    traits::{NomParse, Segment},
};
use nom::{Parser, error::ParseError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThematicBreak<'a>(ThematicBreakSegment<'a>);

impl<'a> ThematicBreak<'a> {
    fn new(segment: ThematicBreakSegment<'a>) -> Self {
        Self(segment)
    }
}

impl<'a> NomParse<'a> for ThematicBreak<'a> {
    fn nom_parse<Error: ParseError<&'a str>>(input: &'a str) -> nom::IResult<&'a str, Self, Error> {
        ThematicBreakSegment::nom_parse.map(Self::new).parse(input)
    }
}

impl<'a> Segment<'a> for ThematicBreak<'a> {
    fn segment(&self) -> &'a str {
        self.0.segment()
    }
}
