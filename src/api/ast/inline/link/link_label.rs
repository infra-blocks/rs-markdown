use crate::api::traits::Segment;

/// A link label, as described in the [spec][https://spec.commonmark.org/0.31.2/#link-label].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkLabel<'a>(&'a str);

impl<'a> LinkLabel<'a> {
    /// Creates a new `LinkLabel` from a `LinkLabelSegment`.
    pub(crate) fn new(segment: &'a str) -> Self {
        Self(segment)
    }
}

impl<'a> Segment<'a> for LinkLabel<'a> {
    fn segment(&self) -> &'a str {
        self.0
    }
}
