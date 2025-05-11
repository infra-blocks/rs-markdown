use crate::parse::{input::NomParse, segment::atx_heading::AtxHeadingSegment, traits::Segment};
use nom::{Parser, error::ParseError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtxHeading<'a>(AtxHeadingSegment<'a>);

impl<'a> AtxHeading<'a> {
    fn new(segment: AtxHeadingSegment<'a>) -> Self {
        Self(segment)
    }

    pub fn level(&self) -> u8 {
        self.0.level()
    }

    pub fn title(&self) -> &'a str {
        self.0.title()
    }
}

impl<'a> NomParse<'a> for AtxHeading<'a> {
    fn nom_parse<Error: ParseError<&'a str>>(input: &'a str) -> nom::IResult<&'a str, Self, Error> {
        AtxHeadingSegment::nom_parse.map(Self::new).parse(input)
    }
}

impl<'a> Segment<'a> for AtxHeading<'a> {
    fn segment(&self) -> &'a str {
        self.0.segment()
    }
}
