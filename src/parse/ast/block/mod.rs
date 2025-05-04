pub mod container;
pub mod leaf;

use leaf::Leaf;
use nom::Parser;

use crate::parse::traits::Parse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Block<'a> {
    Leaf(Leaf<'a>),
}

impl<'a> Parse<'a> for Block<'a> {
    fn parse<Error: nom::error::ParseError<&'a str>>(
        input: &'a str,
    ) -> nom::IResult<&'a str, Self, Error> {
        Leaf::parse.map(Block::Leaf).parse(input)
    }
}
