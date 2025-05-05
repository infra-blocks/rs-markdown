pub mod container;
pub mod leaf;

use crate::parse::traits::{Parse, Segments};
use leaf::Leaf;
use nom::Parser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Block<'a> {
    Leaf(Leaf<'a>),
}

impl<'a> Parse<'a> for Block<'a> {
    fn parse<Error: nom::error::ParseError<&'a str>>(
        input: &'a str,
    ) -> nom::IResult<&'a str, Self, Error> {
        Leaf::parse.map(Block::Leaf).parse(input)
    }
}

impl<'a> Segments<'a> for Block<'a> {
    type SegmentsIter = BlockIterator<'a>;

    fn segments(&'a self) -> Self::SegmentsIter {
        BlockIterator::from(self)
    }
}

pub struct BlockIterator<'a> {
    iter: Box<dyn Iterator<Item = &'a str> + 'a>,
}

impl<'a> BlockIterator<'a> {
    fn new(iter: Box<dyn Iterator<Item = &'a str> + 'a>) -> Self {
        Self { iter }
    }
}

impl<'a> From<&'a Block<'a>> for BlockIterator<'a> {
    fn from(block: &'a Block) -> Self {
        match block {
            Block::Leaf(leaf) => Self::new(Box::new(leaf.segments())),
        }
    }
}

impl<'a> Iterator for BlockIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
