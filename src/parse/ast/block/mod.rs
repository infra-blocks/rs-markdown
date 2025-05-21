pub mod container;
pub mod leaf;

use crate::{
    ast::block::{Block, Leaf},
    parse::{
        input::Input,
        parser::{Map, ParseResult, Parser},
        traits::Parse,
    },
};

impl<'a> Parse<&'a str> for Block<'a> {
    fn parse<I: Input<&'a str>>(input: I) -> ParseResult<I, Self> {
        Leaf::parse.map(Block::Leaf).parse(input)
    }
}
