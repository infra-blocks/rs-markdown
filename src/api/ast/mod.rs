pub mod block;
pub mod inline;

use super::{Segments, ToHtml};
use crate::render::DisplayHtml;
use block::{Block, Leaf, LinkReferenceDefinition};
use std::iter::FusedIterator;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tree<'a> {
    /// TODO: hide those behind an interface.
    pub(crate) blocks: Vec<Block<'a>>,
    pub(crate) link_reference_definitions: Vec<LinkReferenceDefinition<'a>>,
}

impl<'a> Tree<'a> {
    pub(crate) fn new(
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

impl<'a> Segments<'a> for Tree<'a> {
    type SegmentsIter = TreeSegmentsIterator<'a>;

    fn segments(&'a self) -> Self::SegmentsIter {
        TreeSegmentsIterator::from(self)
    }
}

// TODO: statically type this iterator
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

impl ToHtml for Tree<'_> {
    fn to_html(&self) -> String {
        let mut buffer = String::new();
        self.display_html(&mut buffer, &self.link_reference_definitions);
        buffer
    }
}
