mod backticks;
mod tildes;

use crate::parse::traits::ParseLine;
pub use backticks::*;
use parser::{Map, ParseResult, Parser, one_of};
pub use tildes::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FencedCodeOpeningSegment<'a> {
    Backticks(BackticksFencedCodeOpeningSegment<'a>),
    Tildes(TildesFencedCodeOpeningSegment<'a>),
}

pub fn fenced_code_opening_segment<'a>(
    input: &'a str,
) -> ParseResult<&'a str, FencedCodeOpeningSegment<'a>> {
    one_of((
        backticks_fenced_code_opening_segment.map(FencedCodeOpeningSegment::from),
        tildes_fenced_code_opening_segment.map(FencedCodeOpeningSegment::from),
    ))
    .parse(input)
}

impl<'a> ParseLine<'a> for FencedCodeOpeningSegment<'a> {
    fn parse_line(input: &'a str) -> ParseResult<&'a str, Self> {
        fenced_code_opening_segment(input)
    }
}

impl<'a> From<BackticksFencedCodeOpeningSegment<'a>> for FencedCodeOpeningSegment<'a> {
    fn from(segment: BackticksFencedCodeOpeningSegment<'a>) -> Self {
        FencedCodeOpeningSegment::Backticks(segment)
    }
}

impl<'a> From<TildesFencedCodeOpeningSegment<'a>> for FencedCodeOpeningSegment<'a> {
    fn from(segment: TildesFencedCodeOpeningSegment<'a>) -> Self {
        FencedCodeOpeningSegment::Tildes(segment)
    }
}
