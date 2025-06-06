mod leaf;

use crate::parse::blocks::open_block::IBlock;
pub use leaf::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Block<'a> {
    Leaf(Leaf<'a>),
}

impl<'a> IBlock<'a> for Block<'a> {
    type Open = open::Block<'a>;

    fn open(line: &'a str) -> parser::ParseResult<&'a str, Self::Open> {
        if let Ok((remaining, leaf)) = Leaf::open(line) {
            return Ok((remaining, open::Block::Leaf(leaf)));
        }
        panic!("block opening not implemented for line: {}", line);
    }
}

pub mod open {
    use crate::parse::blocks::{block::leaf::open::Leaf, open_block::IOpenBlock};

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Block<'a> {
        Leaf(Leaf<'a>),
    }

    impl<'a> IOpenBlock<'a> for Block<'a> {
        type Closed = super::Block<'a>;

        fn stage(&mut self, line: &'a str) -> Result<&'a str, ()> {
            match self {
                Block::Leaf(open_leaf) => open_leaf.stage(line),
            }
        }

        fn commit(&mut self) {
            match self {
                Block::Leaf(open_leaf) => open_leaf.commit(),
            }
        }

        fn close<F: FnMut(Self::Closed) -> ()>(self, mut sink: F) {
            match self {
                Block::Leaf(open_leaf) => open_leaf.close(|leaf| sink(super::Block::Leaf(leaf))),
            }
        }
    }
}
