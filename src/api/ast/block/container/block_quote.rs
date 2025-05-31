use crate::{Segments, SliceSegments};

// TODO: children?
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockQuote<'a> {
    segments: Vec<&'a str>,
}

impl<'a> BlockQuote<'a> {
    pub(crate) fn new(segments: Vec<&'a str>) -> Self {
        Self { segments }
    }
}

impl<'a> Segments<'a> for BlockQuote<'a> {
    type SegmentsIter = SliceSegments<'a>;

    fn segments(&'a self) -> Self::SegmentsIter {
        self.segments.as_slice().into()
    }
}
