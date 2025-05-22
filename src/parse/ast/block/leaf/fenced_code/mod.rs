mod backticks;
mod tildes;

use crate::{
    ast::block::{BackticksFencedCode, FencedCode, TildesFencedCode},
    parse::{input::Input, traits::Parse},
};
use parser::{Map, ParseResult, Parser, one_of};

impl<'a> Parse<&'a str> for FencedCode<'a> {
    fn parse<I: Input<&'a str>>(input: I) -> ParseResult<I, Self> {
        one_of((
            BackticksFencedCode::parse.map(Self::from),
            TildesFencedCode::parse.map(Self::from),
        ))
        .parse(input)
    }
}
