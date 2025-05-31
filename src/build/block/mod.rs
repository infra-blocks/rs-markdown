mod container;
mod leaf;

use super::{BuildFlow, IBlockBuilder};
use crate::ast::block::Block;
pub use container::*;
pub use leaf::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockBuilder<'a> {
    Container(ContainerBuilder<'a>),
    Leaf(LeafBuilder<'a>),
}

impl<'a> IBlockBuilder<'a> for BlockBuilder<'a> {
    fn maybe_open(line: &'a str) -> Option<Self> {
        if let Some(builder) = ContainerBuilder::maybe_open(line) {
            return Some(Self::Container(builder));
        }
        if let Some(builder) = LeafBuilder::maybe_open(line) {
            return Some(Self::Leaf(builder));
        }
        None
    }

    fn parse_line(&mut self, line: &'a str) -> BuildFlow {
        match self {
            Self::Container(container) => container.parse_line(line),
            Self::Leaf(leaf) => leaf.parse_line(line),
        }
    }

    fn close<E: Extend<Block<'a>>>(self, sink: &mut E) {
        match self {
            Self::Container(container) => container.close(sink),
            Self::Leaf(leaf) => leaf.close(sink),
        }
    }
}

impl<'a> From<ContainerBuilder<'a>> for BlockBuilder<'a> {
    fn from(builder: ContainerBuilder<'a>) -> Self {
        Self::Container(builder)
    }
}

impl<'a> From<ParagraphBuilder<'a>> for BlockBuilder<'a> {
    fn from(builder: ParagraphBuilder<'a>) -> Self {
        LeafBuilder::Paragraph(builder).into()
    }
}

impl<'a> From<LeafBuilder<'a>> for BlockBuilder<'a> {
    fn from(builder: LeafBuilder<'a>) -> Self {
        Self::Leaf(builder)
    }
}
