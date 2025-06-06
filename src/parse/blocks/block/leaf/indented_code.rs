use crate::parse::segment::indented_code::{ContinuationSegments, IndentedCodeSegment};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndentedCode<'a> {
    opening_segment: IndentedCodeSegment<'a>,
    continuation_segments: Option<ContinuationSegments<'a>>,
}

impl<'a> IndentedCode<'a> {
    pub(super) fn new(
        opening_segment: IndentedCodeSegment<'a>,
        continuation_segments: Option<ContinuationSegments<'a>>,
    ) -> Self {
        Self {
            opening_segment,
            continuation_segments,
        }
    }

    pub(super) fn single_segment(opening_segment: IndentedCodeSegment<'a>) -> Self {
        Self::new(opening_segment, None)
    }

    pub(super) fn multi_segments(
        opening_segment: IndentedCodeSegment<'a>,
        continuation_segments: ContinuationSegments<'a>,
    ) -> Self {
        Self::new(opening_segment, Some(continuation_segments))
    }
}
