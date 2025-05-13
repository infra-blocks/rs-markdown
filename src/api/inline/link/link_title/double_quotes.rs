use crate::parse::segment::link_title::DoubleQuotesLinkTitleSegments;

/// The double quotes variant of a link title.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoubleQuotesLinkTitle<'a>(DoubleQuotesLinkTitleSegments<'a>);

impl<'a> DoubleQuotesLinkTitle<'a> {
    pub(crate) fn new(segment: DoubleQuotesLinkTitleSegments<'a>) -> Self {
        Self(segment)
    }
}

// TODO: implement segments
