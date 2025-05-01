use nom::{error::ParseError, Parser};

use crate::parse::segment::atx_heading::AtxHeadingSegment;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtxHeading<'a>(pub AtxHeadingSegment<'a>);

impl<'a> AtxHeading<'a> {
    fn new(segment: AtxHeadingSegment<'a>) -> Self {
        Self(segment)
    }

    pub fn level(&self) -> u8 {
        self.0.level
    }

    pub fn title(&self) -> &'a str {
        self.0.title
    }

    pub fn parser<Error: ParseError<&'a str>>() -> impl Parser<&'a str, Output = Self, Error = Error>
    {
        AtxHeadingSegment::parser().map(Self::new)
    }
}
