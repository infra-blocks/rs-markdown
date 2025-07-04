pub mod container;
pub mod leaf;

use crate::{
    ast::block::{Block, Leaf},
    parse::{input::Input, traits::Parse},
};
use parser::{Map, ParseResult, Parser};

impl<'a> Parse<'a> for Block<'a> {
    fn parse<I: Input<'a>>(input: I) -> ParseResult<I, Self> {
        Leaf::parse.map(Block::Leaf).parse(input)
    }
}
