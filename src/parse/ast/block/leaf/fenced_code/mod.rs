mod backticks;
mod tildes;

pub use backticks::*;
pub use tildes::*;

use nom::{Parser, branch::alt, error::ParseError};
use std::iter::FusedIterator;

use crate::parse::traits::{NomParse, Segments};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FencedCode<'a> {
    Backticks(BackticksFencedCode<'a>),
    Tildes(TildesFencedCode<'a>),
}

impl<'a> FencedCode<'a> {
    pub fn content_segments(&'a self) -> FencedCodeContentSegmentsIterator<'a> {
        self.into()
    }
}

impl<'a> From<BackticksFencedCode<'a>> for FencedCode<'a> {
    fn from(fenced_code: BackticksFencedCode<'a>) -> Self {
        Self::Backticks(fenced_code)
    }
}

impl<'a> From<TildesFencedCode<'a>> for FencedCode<'a> {
    fn from(fenced_code: TildesFencedCode<'a>) -> Self {
        Self::Tildes(fenced_code)
    }
}

impl<'a> NomParse<'a> for FencedCode<'a> {
    fn nom_parse<Error: ParseError<&'a str>>(input: &'a str) -> nom::IResult<&'a str, Self, Error>
    where
        Self: Sized,
    {
        alt((
            BackticksFencedCode::nom_parse.map(Self::from),
            TildesFencedCode::nom_parse.map(Self::from),
        ))
        .parse(input)
    }
}

impl<'a> Segments<'a> for FencedCode<'a> {
    type SegmentsIter = FencedCodeSegmentsIterator<'a>;

    fn segments(&'a self) -> Self::SegmentsIter {
        match self {
            Self::Backticks(backticks) => backticks.segments().into(),
            Self::Tildes(tildes) => tildes.segments().into(),
        }
    }
}

pub enum FencedCodeContentSegmentsIterator<'a> {
    Backticks(BackticksFencedCodeContentSegmentsIterator<'a>),
    Tildes(TildesFencedCodeContentSegmentsIterator<'a>),
}

impl<'a> From<BackticksFencedCodeContentSegmentsIterator<'a>>
    for FencedCodeContentSegmentsIterator<'a>
{
    fn from(iterator: BackticksFencedCodeContentSegmentsIterator<'a>) -> Self {
        Self::Backticks(iterator)
    }
}

impl<'a> From<TildesFencedCodeContentSegmentsIterator<'a>>
    for FencedCodeContentSegmentsIterator<'a>
{
    fn from(iterator: TildesFencedCodeContentSegmentsIterator<'a>) -> Self {
        Self::Tildes(iterator)
    }
}

impl<'a> From<&'a FencedCode<'a>> for FencedCodeContentSegmentsIterator<'a> {
    fn from(fenced_code: &'a FencedCode<'a>) -> Self {
        match fenced_code {
            FencedCode::Backticks(iterator) => iterator.content_segments().into(),
            FencedCode::Tildes(iterator) => iterator.content_segments().into(),
        }
    }
}

impl FusedIterator for FencedCodeContentSegmentsIterator<'_> {}

impl<'a> Iterator for FencedCodeContentSegmentsIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Backticks(iterator) => iterator.next(),
            Self::Tildes(iterator) => iterator.next(),
        }
    }
}

pub enum FencedCodeSegmentsIterator<'a> {
    Backticks(BackticksFencedCodeSegmentsIterator<'a>),
    Tildes(TildesFencedCodeSegmentsIterator<'a>),
}

impl<'a> From<BackticksFencedCodeSegmentsIterator<'a>> for FencedCodeSegmentsIterator<'a> {
    fn from(iterator: BackticksFencedCodeSegmentsIterator<'a>) -> Self {
        Self::Backticks(iterator)
    }
}

impl<'a> From<TildesFencedCodeSegmentsIterator<'a>> for FencedCodeSegmentsIterator<'a> {
    fn from(iterator: TildesFencedCodeSegmentsIterator<'a>) -> Self {
        Self::Tildes(iterator)
    }
}

impl<'a> From<&'a FencedCode<'a>> for FencedCodeSegmentsIterator<'a> {
    fn from(fenced_code: &'a FencedCode<'a>) -> Self {
        match fenced_code {
            FencedCode::Backticks(iterator) => Self::Backticks(iterator.segments()),
            FencedCode::Tildes(iterator) => Self::Tildes(iterator.segments()),
        }
    }
}

impl FusedIterator for FencedCodeSegmentsIterator<'_> {}

impl<'a> Iterator for FencedCodeSegmentsIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Backticks(iterator) => iterator.next(),
            Self::Tildes(iterator) => iterator.next(),
        }
    }
}
