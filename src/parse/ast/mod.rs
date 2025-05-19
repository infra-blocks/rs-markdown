pub mod block;

use super::{
    ParseResult,
    input::Input,
    parser::{Map, Parser, ZeroToMany},
    traits::Parse,
};
use crate::ast::{Block, Tree};

impl<'a> Parse<&'a str> for Tree<'a> {
    fn parse<I: Input<Item = &'a str>>(input: I) -> ParseResult<I, Self> {
        Block::parse.zero_to_many().map(Tree::from).parse(input)
    }
}
