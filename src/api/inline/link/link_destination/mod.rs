mod bracketed;
mod unbracketed;

use crate::api::Segment;
pub use bracketed::*;
pub use unbracketed::*;

/// A link destination, as described in the [spec](https://spec.commonmark.org/0.31.2/#link-destination).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkDestination<'a> {
    Bracketed(BracketedLinkDestination<'a>),
    Unbracketed(UnbracketedLinkDestination<'a>),
}

impl<'a> From<BracketedLinkDestination<'a>> for LinkDestination<'a> {
    fn from(segment: BracketedLinkDestination<'a>) -> Self {
        LinkDestination::Bracketed(segment)
    }
}

impl<'a> From<UnbracketedLinkDestination<'a>> for LinkDestination<'a> {
    fn from(segment: UnbracketedLinkDestination<'a>) -> Self {
        LinkDestination::Unbracketed(segment)
    }
}

impl<'a> Segment<'a> for LinkDestination<'a> {
    fn segment(&self) -> &'a str {
        match self {
            Self::Bracketed(segment) => segment.segment(),
            Self::Unbracketed(segment) => segment.segment(),
        }
    }
}
