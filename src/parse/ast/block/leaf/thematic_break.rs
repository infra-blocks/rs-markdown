use nom::{error::ParseError, Parser};

use crate::parse::{
    segment::thematic_break::ThematicBreakSegment,
    traits::{Parse, Segment},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThematicBreak<'a>(ThematicBreakSegment<'a>);

impl<'a> ThematicBreak<'a> {
    fn new(segment: ThematicBreakSegment<'a>) -> Self {
        Self(segment)
    }
}

impl<'a> Parse<'a> for ThematicBreak<'a> {
    fn parse<Error: ParseError<&'a str>>(input: &'a str) -> nom::IResult<&'a str, Self, Error> {
        ThematicBreakSegment::parse.map(Self::new).parse(input)
    }
}

impl<'a> Segment<'a> for ThematicBreak<'a> {
    fn segment(&self) -> &'a str {
        self.0.segment()
    }
}
