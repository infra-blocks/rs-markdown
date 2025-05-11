pub mod container;
pub mod leaf;

use std::iter::FusedIterator;

use crate::parse::{input::NomParse, traits::Segments};
use leaf::Leaf;
use nom::Parser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Block<'a> {
    Leaf(Leaf<'a>),
}

impl<'a> NomParse<'a> for Block<'a> {
    fn nom_parse<Error: nom::error::ParseError<&'a str>>(
        input: &'a str,
    ) -> nom::IResult<&'a str, Self, Error> {
        Leaf::nom_parse.map(Block::Leaf).parse(input)
    }
}

impl<'a> Segments<'a> for Block<'a> {
    type SegmentsIter = BlockSegmentsIterator<'a>;

    fn segments(&'a self) -> Self::SegmentsIter {
        BlockSegmentsIterator::from(self)
    }
}

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
