mod block;

use block::Block;
use nom::{error::ParseError, multi::many0, Parser};

pub struct Tree<'a> {
    pub blocks: Vec<Block<'a>>,
}

impl<'a> Tree<'a> {
    pub fn new(blocks: Vec<Block<'a>>) -> Self {
        Self { blocks }
    }

    pub fn parser<Error: ParseError<&'a str>>() -> impl Parser<&'a str, Output = Self, Error = Error>
    {
        many0(Block::parser()).map(Self::new)
    }
}
