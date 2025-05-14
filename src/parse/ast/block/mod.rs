pub mod container;
pub mod leaf;

use crate::{
    ast::{Block, Leaf},
    parse::{
        input::{Input, ParseResult},
        parser::{Map, Parser},
        traits::Parse,
    },
};

impl<'a> Parse<&'a str> for Block<'a> {
    fn parse<I: Input<Item = &'a str>>(input: I) -> ParseResult<I, Self> {
        Leaf::parse.map(Block::Leaf).parse(input)
    }
}
