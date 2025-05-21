use crate::api::Segment;

/// The bracketed variant of the link destination.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BracketedLinkDestination<'a>(&'a str);

impl<'a> BracketedLinkDestination<'a> {
    pub(crate) fn new(segment: &'a str) -> Self {
        Self(segment)
    }
}

impl<'a> Segment<'a> for BracketedLinkDestination<'a> {
    fn segment(&self) -> &'a str {
        self.0
    }
}
