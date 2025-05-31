mod container;
mod leaf;

pub use container::*;
pub use leaf::*;

use crate::Segments;
use std::iter::FusedIterator;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Block<'a> {
    Container(Container<'a>),
    Leaf(Leaf<'a>),
}

impl<'a> From<Container<'a>> for Block<'a> {
    fn from(container: Container<'a>) -> Self {
        Self::Container(container)
    }
}

impl<'a> From<BlockQuote<'a>> for Block<'a> {
    fn from(block_quote: BlockQuote<'a>) -> Self {
        Self::Container(block_quote.into())
    }
}

impl<'a> Segments<'a> for Block<'a> {
    type SegmentsIter = BlockSegmentsIterator<'a>;

    fn segments(&'a self) -> Self::SegmentsIter {
        BlockSegmentsIterator::from(self)
    }
}

// TODO: statically type this iterator
pub struct BlockSegmentsIterator<'a> {
    iter: Box<dyn Iterator<Item = &'a str> + 'a>,
}

impl<'a> BlockSegmentsIterator<'a> {
    fn new(iter: Box<dyn Iterator<Item = &'a str> + 'a>) -> Self {
        Self { iter }
    }
}

impl<'a> From<&'a Block<'a>> for BlockSegmentsIterator<'a> {
    fn from(block: &'a Block) -> Self {
        match block {
            Block::Container(container) => Self::new(Box::new(container.segments())),
            Block::Leaf(leaf) => Self::new(Box::new(leaf.segments())),
        }
    }
}

impl FusedIterator for BlockSegmentsIterator<'_> {}

impl<'a> Iterator for BlockSegmentsIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
