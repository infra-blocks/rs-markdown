use nom::{error::ParseError, Parser};

use crate::parse::{segment::blank_line::BlankLineSegment, traits::Parse};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlankLine<'a>(pub BlankLineSegment<'a>);

impl<'a> BlankLine<'a> {
    fn new(segment: BlankLineSegment<'a>) -> Self {
        Self(segment)
    }

    pub fn segment(&self) -> &'a str {
        self.0 .0
    }
}

impl<'a> Parse<'a> for BlankLine<'a> {
    fn parse<Error: ParseError<&'a str>>(input: &'a str) -> nom::IResult<&'a str, Self, Error> {
        BlankLineSegment::parse.map(Self::new).parse(input)
    }
}
