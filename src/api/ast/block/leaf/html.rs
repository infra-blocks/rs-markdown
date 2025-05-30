use crate::{Segments, SliceSegments};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Html<'a> {
    segments: Vec<&'a str>,
}

impl<'a> Html<'a> {
    pub(crate) fn new(segments: Vec<&'a str>) -> Self {
        Self { segments }
    }
}

impl<'a> Segments<'a> for Html<'a> {
    type SegmentsIter = SliceSegments<'a>;

    fn segments(&'a self) -> Self::SegmentsIter {
        self.segments.as_slice().into()
    }
}
