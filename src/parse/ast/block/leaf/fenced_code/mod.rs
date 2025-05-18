mod backticks;
mod tildes;

use crate::{
    ast::{BackticksFencedCode, FencedCode, TildesFencedCode},
    parse::{
        input::{Input, ParseResult},
        parser::{Map, Parser, one_of},
        traits::Parse,
    },
};

impl<'a> Parse<&'a str> for FencedCode<'a> {
    fn parse<I: Input<Item = &'a str>>(input: I) -> ParseResult<I, Self> {
        one_of((
            BackticksFencedCode::parse.map(Self::from),
            TildesFencedCode::parse.map(Self::from),
        ))
        .parse(input)
    }
}
