mod block_quote;

use crate::{
    ast::block::Block,
    parse::phase_1::{block::OpenBlock, traits::IOpenBlock},
};
use block_quote::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenContainer<'a> {
    BlockQuote(OpenBlockQuote<'a>),
}

impl<'a> IOpenBlock<'a> for OpenContainer<'a> {
    fn stage(&mut self, line: &'a str) -> Result<&'a str, ()> {
        match self {
            OpenContainer::BlockQuote(block_quote) => block_quote.stage(line),
        }
    }

    fn commit(&mut self) {
        match self {
            OpenContainer::BlockQuote(block_quote) => block_quote.commit(),
        }
    }

    fn close(self) -> Block<'a> {
        match self {
            OpenContainer::BlockQuote(block_quote) => block_quote.close(),
        }
    }
}

// TODO: make IContainer interface
impl<'a> OpenContainer<'a> {
    pub fn current(&mut self) -> Option<&mut OpenBlock<'a>> {
        match self {
            OpenContainer::BlockQuote(block_quote) => match block_quote.current {
                Some(ref mut current) => Some(current),
                None => None,
            },
        }
    }

    pub fn set_current(&mut self, block: OpenBlock<'a>) {
        match self {
            OpenContainer::BlockQuote(block_quote) => {
                block_quote.current = Some(Box::new(block));
            }
        }
    }
}
