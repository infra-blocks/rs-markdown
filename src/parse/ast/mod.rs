pub mod block;

use super::traits::Parse;
use crate::ast::{Block, Tree};
use nom::{IResult, Parser, multi::many0};

impl<'a> Parse<'a> for Tree<'a> {
    fn parse<Error: nom::error::ParseError<&'a str>>(
        input: &'a str,
    ) -> IResult<&'a str, Self, Error> {
        many0(Block::parse).map(Self::from).parse(input)
    }
}
