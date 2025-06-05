mod container;
mod leaf;

use container::*;
use leaf::*;

use crate::{ast::block::Block, parse::phase_1::traits::IOpenBlock};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenBlock<'a> {
    Container(OpenContainer<'a>),
    Leaf(OpenLeaf<'a>),
}

impl<'a> IOpenBlock<'a> for OpenBlock<'a> {
    fn stage(&mut self, line: &'a str) -> Result<&'a str, ()> {
        match self {
            OpenBlock::Container(container) => container.stage(line),
            OpenBlock::Leaf(leaf) => leaf.stage(line),
        }
    }

    fn commit(&mut self) {
        match self {
            OpenBlock::Container(container) => container.commit(),
            OpenBlock::Leaf(leaf) => leaf.commit(),
        }
    }

    fn close(self) -> Block<'a> {
        match self {
            OpenBlock::Container(container) => container.close(),
            OpenBlock::Leaf(leaf) => leaf.close(),
        }
    }
}
