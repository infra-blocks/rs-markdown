mod block_quote;

use crate::Segments;
pub use block_quote::*;
use std::iter::FusedIterator;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Container<'a> {
    BlockQuote(BlockQuote<'a>),
}

impl<'a> From<BlockQuote<'a>> for Container<'a> {
    fn from(block_quote: BlockQuote<'a>) -> Self {
        Self::BlockQuote(block_quote)
    }
}

// TODO: statically type this iterator.
impl<'a> Segments<'a> for Container<'a> {
    type SegmentsIter = ContainerSegmentsIterator<'a>;

    fn segments(&'a self) -> Self::SegmentsIter {
        self.into()
    }
}

pub struct ContainerSegmentsIterator<'a> {
    iter: Box<dyn Iterator<Item = &'a str> + 'a>,
}

impl<'a> ContainerSegmentsIterator<'a> {
    fn new(iter: Box<dyn Iterator<Item = &'a str> + 'a>) -> Self {
        Self { iter }
    }
}

impl<'a> From<&'a Container<'a>> for ContainerSegmentsIterator<'a> {
    fn from(container: &'a Container) -> Self {
        match container {
            Container::BlockQuote(block_quote) => Self::new(Box::new(block_quote.segments())),
        }
    }
}

impl FusedIterator for ContainerSegmentsIterator<'_> {}

impl<'a> Iterator for ContainerSegmentsIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
