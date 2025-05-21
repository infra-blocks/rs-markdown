pub mod block;
pub mod inline;

use super::{
    input::Input,
    parser::{Map, ParseResult, Parser, ZeroToMany},
    traits::Parse,
};
use crate::ast::{Tree, block::Block};

impl<'a> Parse<&'a str> for Tree<'a> {
    fn parse<I: Input<&'a str>>(input: I) -> ParseResult<I, Self> {
        Block::parse.zero_to_many().map(Tree::from).parse(input)
    }
}
