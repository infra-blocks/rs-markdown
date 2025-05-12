use crate::api::Segment;

/// The double quotes variant of a link title.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoubleQuotesLinkTitle<'a>(&'a str);

impl<'a> DoubleQuotesLinkTitle<'a> {
    pub(crate) fn new(segment: &'a str) -> Self {
        Self(segment)
    }
}

impl<'a> Segment<'a> for DoubleQuotesLinkTitle<'a> {
    fn segment(&self) -> &'a str {
        self.0
    }
}
