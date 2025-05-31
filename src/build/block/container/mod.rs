mod block_quote;

use crate::{ast::block::Block, build::IBlockBuilder};
pub use block_quote::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContainerBuilder<'a> {
    BlockQuote(BlockQuoteBuilder<'a>),
}

impl<'a> IBlockBuilder<'a> for ContainerBuilder<'a> {
    fn maybe_open(line: &'a str) -> Option<Self> {
        if let Some(builder) = BlockQuoteBuilder::maybe_open(line) {
            Some(Self::BlockQuote(builder))
        } else {
            None
        }
    }

    fn parse_line(&mut self, line: &'a str) -> crate::build::BuildFlow {
        match self {
            Self::BlockQuote(builder) => builder.parse_line(line),
        }
    }

    fn close<E: Extend<Block<'a>>>(self, sink: &mut E) {
        match self {
            Self::BlockQuote(builder) => builder.close(sink),
        }
    }
}
