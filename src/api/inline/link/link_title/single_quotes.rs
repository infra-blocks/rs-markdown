use crate::parse::segment::link_title::SingleQuotesLinkTitleSegments;

/// The single quotes variant of a link title.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SingleQuotesLinkTitle<'a>(SingleQuotesLinkTitleSegments<'a>);

impl<'a> SingleQuotesLinkTitle<'a> {
    pub(crate) fn new(segment: SingleQuotesLinkTitleSegments<'a>) -> Self {
        Self(segment)
    }
}

// TODO: implement segments
