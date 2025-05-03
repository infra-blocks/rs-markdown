use nom::{error::ParseError, Parser};

use crate::parse::{segment::atx_heading::AtxHeadingSegment, traits::Parse};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtxHeading<'a>(pub AtxHeadingSegment<'a>);

impl<'a> AtxHeading<'a> {
    fn new(segment: AtxHeadingSegment<'a>) -> Self {
        Self(segment)
    }

    pub fn level(&self) -> u8 {
        self.0.level
    }

    pub fn segment(&self) -> &'a str {
        self.0.segment
    }

    pub fn title(&self) -> &'a str {
        self.0.title
    }
}

impl<'a> Parse<'a> for AtxHeading<'a> {
    fn parse<Error: ParseError<&'a str>>(input: &'a str) -> nom::IResult<&'a str, Self, Error> {
        AtxHeadingSegment::parse.map(Self::new).parse(input)
    }
}
