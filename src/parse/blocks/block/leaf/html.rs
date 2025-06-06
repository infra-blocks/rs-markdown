use crate::{Segments, SliceSegments};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Html<'a> {
    segments: Vec<&'a str>,
}

impl<'a> Html<'a> {
    pub(super) fn new(segments: Vec<&'a str>) -> Self {
        Self { segments }
    }
}
