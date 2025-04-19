mod container;
mod leaf;

use leaf::Leaf;
use nom::Parser;

pub enum Block<'a> {
    Leaf(Leaf<'a>),
}

impl<'a> Block<'a> {
    pub fn parser<Error: nom::error::ParseError<&'a str>>(
    ) -> impl nom::Parser<&'a str, Output = Self, Error = Error> {
        Leaf::parser().map(Block::Leaf)
    }
}
