pub mod block;
pub mod inline;

use super::{input::Input, traits::Parse};
use crate::ast::{Document, block::Block};
use parser::{Map, ParseResult, Parser, Repeated};

impl<'a> Parse<'a> for Document<'a> {
    fn parse<I: Input<'a>>(input: I) -> ParseResult<I, Self> {
        Block::parse.repeated().map(Document::from).parse(input)
    }
}
