use crate::{
    Segments,
    parse::segment::link_title::{
        DoubleQuotesLinkTitleMultiSegments, DoubleQuotesLinkTitleSingleSegment,
    },
};
use std::iter::FusedIterator;

/// The double quotes variant of a link title.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DoubleQuotesLinkTitle<'a> {
    Single(DoubleQuotesLinkTitleSingleSegment<'a>),
    Multi(DoubleQuotesLinkTitleMultiSegments<'a>),
}

impl<'a> From<DoubleQuotesLinkTitleSingleSegment<'a>> for DoubleQuotesLinkTitle<'a> {
    fn from(title: DoubleQuotesLinkTitleSingleSegment<'a>) -> Self {
        DoubleQuotesLinkTitle::Single(title)
    }
}

impl<'a> From<DoubleQuotesLinkTitleMultiSegments<'a>> for DoubleQuotesLinkTitle<'a> {
    fn from(title: DoubleQuotesLinkTitleMultiSegments<'a>) -> Self {
        DoubleQuotesLinkTitle::Multi(title)
    }
}

impl<'a> Segments<'a> for DoubleQuotesLinkTitle<'a> {
    type SegmentsIter = DoubleQuotesLinkTitleSegmentsIter<'a>;

    fn segments(&'a self) -> Self::SegmentsIter {
        self.into()
    }
}

pub enum DoubleQuotesLinkTitleSegmentsIter<'a> {
    Single(<DoubleQuotesLinkTitleSingleSegment<'a> as Segments<'a>>::SegmentsIter),
    Multi(<DoubleQuotesLinkTitleMultiSegments<'a> as Segments<'a>>::SegmentsIter),
}

impl<'a> From<&'a DoubleQuotesLinkTitle<'a>> for DoubleQuotesLinkTitleSegmentsIter<'a> {
    fn from(title: &'a DoubleQuotesLinkTitle<'a>) -> Self {
        match title {
            DoubleQuotesLinkTitle::Single(single) => {
                DoubleQuotesLinkTitleSegmentsIter::Single(single.segments())
            }
            DoubleQuotesLinkTitle::Multi(multi) => {
                DoubleQuotesLinkTitleSegmentsIter::Multi(multi.segments())
            }
        }
    }
}

impl FusedIterator for DoubleQuotesLinkTitleSegmentsIter<'_> {}

impl<'a> Iterator for DoubleQuotesLinkTitleSegmentsIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            DoubleQuotesLinkTitleSegmentsIter::Single(iter) => iter.next(),
            DoubleQuotesLinkTitleSegmentsIter::Multi(iter) => iter.next(),
        }
    }
}
