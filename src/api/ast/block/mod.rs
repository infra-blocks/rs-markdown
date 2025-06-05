mod container;
mod leaf;

pub use leaf::*;

use crate::Segments;
use std::iter::FusedIterator;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Block<'a> {
    Container(Container<'a>),
    Leaf(Leaf<'a>),
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
