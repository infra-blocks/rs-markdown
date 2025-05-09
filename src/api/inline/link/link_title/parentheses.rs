use crate::api::Segment;

/// The parentheses variant of a link title.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParenthesesLinkTitle<'a>(&'a str);

impl<'a> ParenthesesLinkTitle<'a> {
    pub(crate) fn new(segment: &'a str) -> Self {
        Self(segment)
    }
}

impl<'a> Segment<'a> for ParenthesesLinkTitle<'a> {
    fn segment(&self) -> &'a str {
        self.0
    }
}
