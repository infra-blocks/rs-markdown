pub mod block;
pub mod inline;

use super::{input::NomParse, traits::Segments};
use block::{
    Block,
    leaf::{Leaf, link_reference_definition::LinkReferenceDefinition},
};
use nom::{IResult, Parser, multi::many0};
use std::iter::FusedIterator;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tree<'a> {
    pub blocks: Vec<Block<'a>>,
    pub link_reference_definitions: Vec<LinkReferenceDefinition<'a>>,
}

impl<'a> Tree<'a> {
    pub fn new(
        blocks: Vec<Block<'a>>,
        link_reference_definitions: Vec<LinkReferenceDefinition<'a>>,
    ) -> Self {
        Self {
            blocks,
            link_reference_definitions,
        }
    }
}

impl<'a> From<Vec<Block<'a>>> for Tree<'a> {
    fn from(blocks: Vec<Block<'a>>) -> Self {
        let mut link_reference_definitions = vec![];
        for block in &blocks {
            let Block::Leaf(leaf) = block;
            if let Leaf::LinkReferenceDefinition(link_reference_definition) = leaf {
                link_reference_definitions.push(link_reference_definition.clone());
            }
        }

        Self::new(blocks, link_reference_definitions)
    }
}

impl<'a> NomParse<'a> for Tree<'a> {
    fn nom_parse<Error: nom::error::ParseError<&'a str>>(
        input: &'a str,
    ) -> IResult<&'a str, Self, Error> {
        many0(Block::nom_parse).map(Self::from).parse(input)
    }
}

impl<'a> Segments<'a> for Tree<'a> {
    type SegmentsIter = TreeSegmentsIterator<'a>;

    fn segments(&'a self) -> Self::SegmentsIter {
        TreeSegmentsIterator::from(self)
    }
}

pub struct TreeSegmentsIterator<'a> {
    iter: Box<dyn Iterator<Item = &'a str> + 'a>,
}

impl<'a> TreeSegmentsIterator<'a> {
    fn new(iter: Box<dyn Iterator<Item = &'a str> + 'a>) -> Self {
        Self { iter }
    }
}

impl<'a> From<&'a Tree<'a>> for TreeSegmentsIterator<'a> {
    fn from(tree: &'a Tree) -> Self {
        let iter = tree.blocks.iter().flat_map(|block| block.segments());
        Self::new(Box::new(iter))
    }
}

impl FusedIterator for TreeSegmentsIterator<'_> {}

impl<'a> Iterator for TreeSegmentsIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
