mod backticks;
mod tildes;

use crate::{
    ast::{BackticksFencedCode, FencedCode, TildesFencedCode},
    parse::traits::Parse,
};
use nom::{Parser, branch::alt, error::ParseError};

impl<'a> Parse<'a> for FencedCode<'a> {
    fn parse<Error: ParseError<&'a str>>(input: &'a str) -> nom::IResult<&'a str, Self, Error>
    where
        Self: Sized,
    {
        alt((
            BackticksFencedCode::parse.map(Self::from),
            TildesFencedCode::parse.map(Self::from),
        ))
        .parse(input)
    }
}
