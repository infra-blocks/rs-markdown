use super::traits::NomParse;
use nom::error::Error;
use std::usize;

pub type ParseResult<I, T> = Result<(I, T), I>;

pub trait Parser<I> {
    type Output;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output>;
}

impl<I, T, F> Parser<I> for F
where
    F: Fn(I) -> ParseResult<I, T>,
{
    type Output = T;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output> {
        self(input)
    }
}

pub enum InputConsumption {
    Bytes(usize),
    Segments(usize),
}

pub fn bytes(count: usize) -> InputConsumption {
    InputConsumption::Bytes(count)
}

pub fn segments(count: usize) -> InputConsumption {
    InputConsumption::Segments(count)
}

pub trait Input
where
    Self: Sized,
{
    type Segment;

    /// Returns the first segment of the input.
    fn segment(&self) -> Self::Segment;
    /// Returns an iterator over the segments of the input.
    fn segments(&self) -> impl Iterator<Item = Self::Segment>;

    /// Returns the remaining input after consuming the specified amount.
    fn consumed(self, consumption: InputConsumption) -> Self;
    /// Returns whether the input is empty.
    fn is_empty(&self) -> bool;
    /// Returns the amount of bytes between the start of this input and the start of the given segment.
    ///
    /// Panics if the segment is not part of this input.
    fn offset(&self, segment: Self::Segment) -> usize;
    /// A convenience method to return a [ParseResult] with the reamining input properly set.
    fn parsed<T>(self, consumption: InputConsumption, value: T) -> ParseResult<Self, T> {
        Ok((self.consumed(consumption), value))
    }
    /// A convenience method to return a [ParseResult] upon failure.
    fn failed<T>(self) -> ParseResult<Self, T> {
        Err(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinesInput<'a> {
    input: &'a str,
    offset: usize,
}

impl<'a> LinesInput<'a> {
    unsafe fn new(input: &'a str, offset: usize) -> Self {
        if offset > input.len() {
            panic!("invalid offset: {} > input length: {}", offset, input.len());
        }
        Self { input, offset }
    }

    pub fn remaining(&self) -> &'a str {
        self.remaining_at(self.offset)
    }

    fn remaining_at(&self, offset: usize) -> &'a str {
        &self.input[offset..]
    }

    fn segments_to_bytes(&self, count: usize) -> usize {
        // TODO: check count is not greater than the number of segments.
        self.segments().take(count).map(|s| s.len()).sum()
    }

    fn validate_offset(&self, offset: usize) -> usize {
        if self.input.is_char_boundary(offset) {
            offset
        } else {
            panic!("invalid offset: {}, not a char boundary", offset);
        }
    }
}

impl<'a> From<&'a str> for LinesInput<'a> {
    fn from(input: &'a str) -> Self {
        unsafe { Self::new(input, 0) }
    }
}

impl<'a> Input for LinesInput<'a> {
    type Segment = &'a str;

    fn segment(&self) -> &'a str {
        self.remaining().split_inclusive("\n").next().unwrap_or("")
    }

    fn segments(&self) -> impl Iterator<Item = &'a str> {
        self.remaining().split_inclusive("\n")
    }

    fn consumed(self, consumption: InputConsumption) -> Self {
        match consumption {
            InputConsumption::Bytes(count) => {
                let offset = self.validate_offset(self.offset + count);
                unsafe { LinesInput::new(self.input, offset) }
            }
            InputConsumption::Segments(count) => {
                let offset = self.validate_offset(self.offset + self.segments_to_bytes(count));
                unsafe { LinesInput::new(self.input, offset) }
            }
        }
    }

    fn offset(&self, segment: Self::Segment) -> usize {
        let remaining = self.remaining();
        let remaining_start_offset = remaining.as_ptr() as usize;
        let remaining_stop_offset = remaining_start_offset + remaining.len();
        let segment_start_offset = segment.as_ptr() as usize;
        let segment_stop_offset = segment_start_offset + segment.len();

        if (segment_start_offset < remaining_start_offset)
            || (segment_stop_offset > remaining_stop_offset)
        {
            panic!(
                "invalid offset calculation: segment [{},{}] is not within bounds [{},{}]",
                segment_start_offset,
                segment_stop_offset,
                remaining_start_offset,
                remaining_stop_offset
            );
        }

        segment_start_offset - remaining_start_offset
    }

    fn is_empty(&self) -> bool {
        self.offset == self.input.len()
    }
}

pub trait Parse<I>
where
    I: Input,
    Self: Sized,
{
    fn parse(input: I) -> ParseResult<I, Self>;
}

impl<'a, I, T> Parse<I> for T
where
    I: Input<Segment = &'a str>,
    T: NomParse<'a>,
{
    fn parse(input: I) -> ParseResult<I, Self> {
        match T::nom_parse::<Error<&str>>(input.segment()) {
            Ok((remaining, parsed)) => {
                let bytes_consumed = input.offset(remaining);
                input.parsed(bytes(bytes_consumed), parsed)
            }
            Err(_) => input.failed(),
        }
    }
}

pub trait ParseWholeSegment<S>
where
    Self: Sized,
{
    fn parse_whole_segment(segment: S) -> Option<Self>;
}

impl<'a, T> ParseWholeSegment<&'a str> for T
where
    T: NomParse<'a>,
{
    fn parse_whole_segment(segment: &'a str) -> Option<Self> {
        match T::nom_parse::<Error<&str>>(segment) {
            Ok((remaining, parsed)) if remaining.is_empty() => Some(parsed),
            _ => None,
        }
    }
}
