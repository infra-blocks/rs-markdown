pub mod block;

use block::{
    leaf::{link_reference_definition::LinkReferenceDefinition, Leaf},
    Block,
};
use nom::{multi::many0, IResult, Parser};

use super::traits::Parse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tree<'a> {
    pub blocks: Vec<Block<'a>>,
    pub link_reference_definitions: Vec<LinkReferenceDefinition<'a>>,
}

impl<'a> Tree<'a> {
    pub fn new(
        blocks: Vec<Block<'a>>,
        link_reference_definitions: Vec<LinkReferenceDefinition<'a>>,
    ) -> Self {
        Self {
            blocks,
            link_reference_definitions,
        }
    }
}

impl<'a> Parse<'a> for Tree<'a> {
    fn parse<Error: nom::error::ParseError<&'a str>>(
        input: &'a str,
    ) -> IResult<&'a str, Self, Error> {
        many0(Block::parse).map(Self::from).parse(input)
    }
}

impl<'a> From<Vec<Block<'a>>> for Tree<'a> {
    fn from(blocks: Vec<Block<'a>>) -> Self {
        let mut link_reference_definitions = vec![];
        for block in &blocks {
            let Block::Leaf(leaf) = block;
            if let Leaf::LinkReferenceDefinition(link_reference_definition) = leaf {
                link_reference_definitions.push(link_reference_definition.clone());
            }
        }

        Self::new(blocks, link_reference_definitions)
    }
}
