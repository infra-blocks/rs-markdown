pub mod container;
pub mod leaf;

use crate::{
    ast::{Block, Leaf},
    parse::traits::Parse,
};
use nom::Parser;

impl<'a> Parse<'a> for Block<'a> {
    fn parse<Error: nom::error::ParseError<&'a str>>(
        input: &'a str,
    ) -> nom::IResult<&'a str, Self, Error> {
        Leaf::parse.map(Block::Leaf).parse(input)
    }
}
