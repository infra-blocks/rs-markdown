use nom::{error::ParseError, Parser};

use crate::parse::segment::blank_line::BlankLineSegment;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlankLine<'a>(pub BlankLineSegment<'a>);

impl<'a> BlankLine<'a> {
    fn new(segment: BlankLineSegment<'a>) -> Self {
        Self(segment)
    }

    pub fn parser<Error: ParseError<&'a str>>() -> impl Parser<&'a str, Output = Self, Error = Error>
    {
        BlankLineSegment::parser().map(Self::new)
    }
}
