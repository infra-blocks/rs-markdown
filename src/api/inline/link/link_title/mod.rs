mod double_quotes;
mod parentheses;
mod single_quotes;

use crate::api::Segment;
pub use double_quotes::*;
pub use parentheses::*;
pub use single_quotes::*;

/// A link title, as described in the [spec](https://spec.commonmark.org/0.31.2/#link-title).
#[derive(Debug, Clone, PartialEq)]
pub enum LinkTitle<'a> {
    DoubleQuotes(DoubleQuotesLinkTitle<'a>),
    Parentheses(ParenthesesLinkTitle<'a>),
    SingleQuotes(SingleQuotesLinkTitle<'a>),
}

impl<'a> From<DoubleQuotesLinkTitle<'a>> for LinkTitle<'a> {
    fn from(title: DoubleQuotesLinkTitle<'a>) -> Self {
        LinkTitle::DoubleQuotes(title)
    }
}

impl<'a> From<SingleQuotesLinkTitle<'a>> for LinkTitle<'a> {
    fn from(title: SingleQuotesLinkTitle<'a>) -> Self {
        LinkTitle::SingleQuotes(title)
    }
}

impl<'a> From<ParenthesesLinkTitle<'a>> for LinkTitle<'a> {
    fn from(title: ParenthesesLinkTitle<'a>) -> Self {
        LinkTitle::Parentheses(title)
    }
}

impl<'a> Segment<'a> for LinkTitle<'a> {
    fn segment(&self) -> &'a str {
        match self {
            Self::DoubleQuotes(segment) => segment.segment(),
            Self::Parentheses(segment) => segment.segment(),
            Self::SingleQuotes(segment) => segment.segment(),
        }
    }
}
