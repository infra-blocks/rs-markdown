mod paragraph;
use crate::{
    ast::block::Block,
    build::{BuildFlow, IBlockBuilder},
};
pub use paragraph::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LeafBuilder<'a> {
    Paragraph(ParagraphBuilder<'a>),
}

impl<'a> From<ParagraphBuilder<'a>> for LeafBuilder<'a> {
    fn from(builder: ParagraphBuilder<'a>) -> Self {
        Self::Paragraph(builder)
    }
}

impl<'a> IBlockBuilder<'a> for LeafBuilder<'a> {
    fn maybe_open(line: &'a str) -> Option<Self> {
        if let Some(builder) = ParagraphBuilder::maybe_open(line) {
            Some(builder.into())
        } else {
            None
        }
    }

    fn parse_line(&mut self, line: &'a str) -> BuildFlow {
        match self {
            Self::Paragraph(builder) => builder.parse_line(line),
        }
    }

    fn close<E: Extend<Block<'a>>>(self, sink: &mut E) {
        match self {
            Self::Paragraph(builder) => builder.close(sink),
        }
    }
}
