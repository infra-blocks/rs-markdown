use nom::{error::ParseError, Parser};

use crate::parse::segment::thematic_break::ThematicBreakSegment;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThematicBreak<'a>(pub ThematicBreakSegment<'a>);

impl<'a> ThematicBreak<'a> {
    fn new(segment: ThematicBreakSegment<'a>) -> Self {
        Self(segment)
    }

    pub fn segment(&self) -> &'a str {
        self.0 .0
    }

    pub fn parser<Error: ParseError<&'a str>>() -> impl Parser<&'a str, Output = Self, Error = Error>
    {
        ThematicBreakSegment::parser().map(Self::new)
    }
}
