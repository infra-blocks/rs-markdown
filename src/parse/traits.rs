use std::{fmt::Debug, iter};

use nom::{error::ParseError, IResult};

pub trait Parse<'a> {
    fn parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error>
    where
        Self: Sized;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseWholeError<'a, E: ParseError<&'a str>> {
    RemainingInput(&'a str),
    NomError(nom::Err<E>),
}

pub trait ParseWhole<'a> {
    fn parse_whole<Error: ParseError<&'a str>>(
        input: &'a str,
    ) -> Result<Self, ParseWholeError<'a, Error>>
    where
        Self: Sized;
}

impl<'a, T> ParseWhole<'a> for T
where
    T: Parse<'a>,
{
    fn parse_whole<Error: ParseError<&'a str>>(
        input: &'a str,
    ) -> Result<Self, ParseWholeError<'a, Error>> {
        match Self::parse(input) {
            Ok((remaining, result)) => {
                if remaining.is_empty() {
                    Ok(result)
                } else {
                    Err(ParseWholeError::RemainingInput(remaining))
                }
            }
            Err(err) => Err(ParseWholeError::NomError(err)),
        }
    }
}

/// This trait is for struct can be represented as a single segment.
///
/// Those can be blocks that are single segment by definition, such as [crate::parse::ast::block::leaf::atx_heading::AtxHeading],
/// or straight up segment types, such as [crate::parse::segment::atx_heading::AtxHeadingSegment].
pub trait Segment<'a> {
    /// Returns the single segment of the block.
    fn segment(&self) -> &'a str;
}

pub trait Segments<'a> {
    type SegmentsIter: Iterator<Item = &'a str>;

    /// Returns the segments of the block.
    fn segments(&'a self) -> Self::SegmentsIter;
}

impl<'a, T> Segments<'a> for T
where
    T: Segment<'a>,
{
    type SegmentsIter = iter::Once<&'a str>;

    fn segments(&self) -> Self::SegmentsIter {
        iter::once(self.segment())
    }
}

// TODO: remove.
// TODO: turn this into static typing with associated type. I.E. type Iter = Iterator<Item = &'a str>;
// All AST block nodes implement the [Text] trait, which allows to iterate over the text
// that composes the block.
/* pub trait Text<'a> {
    /// Returns an iterator over the text segments of the block.
    fn text(&'a self) -> impl Iterator<Item = &'a str>;
}

impl<'a, T> Text<'a> for T
where
    T: Segment<'a>,
{
    fn text(&'a self) -> impl Iterator<Item = &'a str> {
        iter::once(self.segment())
    }
}
 */
