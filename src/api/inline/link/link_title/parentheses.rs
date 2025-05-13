use crate::{
    Segments,
    parse::segment::link_title::{
        ParenthesesLinkTitleMultiSegments, ParenthesesLinkTitleSingleSegment,
    },
};
use std::iter::FusedIterator;

/// The parentheses variant of a link title.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParenthesesLinkTitle<'a> {
    Single(ParenthesesLinkTitleSingleSegment<'a>),
    Multi(ParenthesesLinkTitleMultiSegments<'a>),
}

impl<'a> From<ParenthesesLinkTitleSingleSegment<'a>> for ParenthesesLinkTitle<'a> {
    fn from(title: ParenthesesLinkTitleSingleSegment<'a>) -> Self {
        ParenthesesLinkTitle::Single(title)
    }
}
impl<'a> From<ParenthesesLinkTitleMultiSegments<'a>> for ParenthesesLinkTitle<'a> {
    fn from(title: ParenthesesLinkTitleMultiSegments<'a>) -> Self {
        ParenthesesLinkTitle::Multi(title)
    }
}

impl<'a> Segments<'a> for ParenthesesLinkTitle<'a> {
    type SegmentsIter = ParenthesesLinkTitleSegmentsIter<'a>;

    fn segments(&'a self) -> Self::SegmentsIter {
        self.into()
    }
}

pub enum ParenthesesLinkTitleSegmentsIter<'a> {
    Single(<ParenthesesLinkTitleSingleSegment<'a> as Segments<'a>>::SegmentsIter),
    Multi(<ParenthesesLinkTitleMultiSegments<'a> as Segments<'a>>::SegmentsIter),
}

impl<'a> From<&'a ParenthesesLinkTitle<'a>> for ParenthesesLinkTitleSegmentsIter<'a> {
    fn from(title: &'a ParenthesesLinkTitle<'a>) -> Self {
        match title {
            ParenthesesLinkTitle::Single(single) => {
                ParenthesesLinkTitleSegmentsIter::Single(single.segments())
            }
            ParenthesesLinkTitle::Multi(multi) => {
                ParenthesesLinkTitleSegmentsIter::Multi(multi.segments())
            }
        }
    }
}

impl FusedIterator for ParenthesesLinkTitleSegmentsIter<'_> {}

impl<'a> Iterator for ParenthesesLinkTitleSegmentsIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ParenthesesLinkTitleSegmentsIter::Single(iter) => iter.next(),
            ParenthesesLinkTitleSegmentsIter::Multi(iter) => iter.next(),
        }
    }
}
