use super::{IndentedCodeOrBlankLineSegment, IndentedCodeSegment};
use crate::{
    ast::block::BlankLine,
    parse::{input::Input, traits::Parse},
};
use parser::{And, OneToMany, ParseResult, Parser, ZeroToMany};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContinuationSegments<'a> {
    pub segments: Vec<IndentedCodeOrBlankLineSegment<'a>>,
    pub closing_segment: IndentedCodeSegment<'a>,
}

impl<'a> ContinuationSegments<'a> {
    pub(crate) fn new(
        segments: Vec<IndentedCodeOrBlankLineSegment<'a>>,
        closing_segment: IndentedCodeSegment<'a>,
    ) -> Self {
        Self {
            segments,
            closing_segment,
        }
    }
}

impl<'a> Parse<&'a str> for ContinuationSegments<'a> {
    fn parse<I: Input<&'a str>>(input: I) -> ParseResult<I, Self> {
        let (remaining, blocks) = BlankLine::parse
            .zero_to_many()
            .and(IndentedCodeSegment::parse)
            .one_to_many()
            .parse(input)?;

        let mut segments = Vec::new();
        for (blank_lines, indented_code_segment) in blocks {
            segments.extend(
                blank_lines
                    .into_iter()
                    .map(IndentedCodeOrBlankLineSegment::from),
            );
            segments.push(IndentedCodeOrBlankLineSegment::from(indented_code_segment));
        }
        // The last segment is guaranteed to be an indented code segment given our algorithm.
        let closing_segment = segments.pop().unwrap().unwrap_indented_code();
        let continuation_segments = ContinuationSegments::new(segments, closing_segment);
        Ok((remaining, continuation_segments))
    }
}
