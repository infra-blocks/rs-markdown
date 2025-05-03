use nom::{error::ParseError, Parser};

use crate::parse::{segment::thematic_break::ThematicBreakSegment, traits::Parse};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThematicBreak<'a>(pub ThematicBreakSegment<'a>);

impl<'a> ThematicBreak<'a> {
    fn new(segment: ThematicBreakSegment<'a>) -> Self {
        Self(segment)
    }

    pub fn segment(&self) -> &'a str {
        self.0 .0
    }
}

impl<'a> Parse<'a> for ThematicBreak<'a> {
    fn parse<Error: ParseError<&'a str>>(input: &'a str) -> nom::IResult<&'a str, Self, Error> {
        ThematicBreakSegment::parse.map(Self::new).parse(input)
    }
}
