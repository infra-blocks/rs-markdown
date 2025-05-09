use crate::api::Segment;

/// The single quotes variant of a link title.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SingleQuotesLinkTitle<'a>(&'a str);

impl<'a> SingleQuotesLinkTitle<'a> {
    pub(crate) fn new(segment: &'a str) -> Self {
        Self(segment)
    }
}

impl<'a> Segment<'a> for SingleQuotesLinkTitle<'a> {
    fn segment(&self) -> &'a str {
        self.0
    }
}
