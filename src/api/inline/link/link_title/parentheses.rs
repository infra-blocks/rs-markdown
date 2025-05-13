use crate::parse::segment::link_title::ParenthesesLinkTitleSegments;

/// The parentheses variant of a link title.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParenthesesLinkTitle<'a>(ParenthesesLinkTitleSegments<'a>);

impl<'a> ParenthesesLinkTitle<'a> {
    pub(crate) fn new(segment: ParenthesesLinkTitleSegments<'a>) -> Self {
        Self(segment)
    }
}

// TODO: implement segments
