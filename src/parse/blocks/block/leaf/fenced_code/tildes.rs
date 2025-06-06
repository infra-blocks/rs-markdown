use crate::parse::segment::fenced_code::{
    TildesFencedCodeClosingSegment, TildesFencedCodeOpeningSegment,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TildesFencedCode<'a> {
    opening_segment: TildesFencedCodeOpeningSegment<'a>,
    content_segments: Vec<&'a str>,
    /// The closing segment is allowed to be None in one scenario: when the end of input is reached
    /// before a closing segment. This is allowed by the spec.
    closing_segment: Option<TildesFencedCodeClosingSegment<'a>>,
}

impl<'a> TildesFencedCode<'a> {
    pub(super) fn new(
        opening_segment: TildesFencedCodeOpeningSegment<'a>,
        content_segments: Vec<&'a str>,
        closing_segment: Option<TildesFencedCodeClosingSegment<'a>>,
    ) -> Self {
        Self {
            opening_segment,
            content_segments,
            closing_segment,
        }
    }
}
