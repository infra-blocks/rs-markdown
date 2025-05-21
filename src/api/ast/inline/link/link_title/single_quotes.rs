use crate::{
    Segments,
    parse::segment::link_title::{
        SingleQuotesLinkTitleMultiSegments, SingleQuotesLinkTitleSingleSegment,
    },
};
use std::iter::FusedIterator;

/// The single quotes variant of a link title.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SingleQuotesLinkTitle<'a> {
    Single(SingleQuotesLinkTitleSingleSegment<'a>),
    Multi(SingleQuotesLinkTitleMultiSegments<'a>),
}

impl<'a> From<SingleQuotesLinkTitleSingleSegment<'a>> for SingleQuotesLinkTitle<'a> {
    fn from(title: SingleQuotesLinkTitleSingleSegment<'a>) -> Self {
        SingleQuotesLinkTitle::Single(title)
    }
}

impl<'a> From<SingleQuotesLinkTitleMultiSegments<'a>> for SingleQuotesLinkTitle<'a> {
    fn from(title: SingleQuotesLinkTitleMultiSegments<'a>) -> Self {
        SingleQuotesLinkTitle::Multi(title)
    }
}

impl<'a> Segments<'a> for SingleQuotesLinkTitle<'a> {
    type SegmentsIter = SingleQuotesLinkTitleSegmentsIter<'a>;

    fn segments(&'a self) -> Self::SegmentsIter {
        self.into()
    }
}

pub enum SingleQuotesLinkTitleSegmentsIter<'a> {
    Single(<SingleQuotesLinkTitleSingleSegment<'a> as Segments<'a>>::SegmentsIter),
    Multi(<SingleQuotesLinkTitleMultiSegments<'a> as Segments<'a>>::SegmentsIter),
}

impl<'a> From<&'a SingleQuotesLinkTitle<'a>> for SingleQuotesLinkTitleSegmentsIter<'a> {
    fn from(title: &'a SingleQuotesLinkTitle<'a>) -> Self {
        match title {
            SingleQuotesLinkTitle::Single(single) => {
                SingleQuotesLinkTitleSegmentsIter::Single(single.segments())
            }
            SingleQuotesLinkTitle::Multi(multi) => {
                SingleQuotesLinkTitleSegmentsIter::Multi(multi.segments())
            }
        }
    }
}

impl FusedIterator for SingleQuotesLinkTitleSegmentsIter<'_> {}

impl<'a> Iterator for SingleQuotesLinkTitleSegmentsIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            SingleQuotesLinkTitleSegmentsIter::Single(iter) => iter.next(),
            SingleQuotesLinkTitleSegmentsIter::Multi(iter) => iter.next(),
        }
    }
}
