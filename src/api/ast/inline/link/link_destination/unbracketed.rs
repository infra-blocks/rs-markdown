use crate::api::Segment;

/// The unbracketed variant of the link destination.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnbracketedLinkDestination<'a>(&'a str);

impl<'a> UnbracketedLinkDestination<'a> {
    pub(crate) fn new(segment: &'a str) -> Self {
        Self(segment)
    }
}

impl<'a> Segment<'a> for UnbracketedLinkDestination<'a> {
    fn segment(&self) -> &'a str {
        self.0
    }
}
