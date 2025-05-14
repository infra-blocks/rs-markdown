use super::{Parser, ParserMut, ParserOnce};
use crate::parse::input::{Input, ParseResult};

pub trait Validate<F>: Sized {
    fn validate(self, func: F) -> ValidateParser<Self, F>;
}

impl<F, T> Validate<F> for T {
    fn validate(self, func: F) -> ValidateParser<Self, F> {
        validate(self, func)
    }
}

pub fn validate<P, F>(parser: P, func: F) -> ValidateParser<P, F> {
    ValidateParser::new(parser, func)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidateParser<P, F> {
    parser: P,
    func: F,
}

impl<P, F> ValidateParser<P, F> {
    fn new(parser: P, func: F) -> Self {
        Self { parser, func }
    }
}

impl<I, P, F> Parser<I> for ValidateParser<P, F>
where
    I: Input,
    P: Parser<I>,
    F: Fn(&P::Output) -> bool,
{
    type Output = P::Output;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output> {
        let (remaining, parsed) = self.parser.parse(input.clone())?;
        if (self.func)(&parsed) {
            Ok((remaining, parsed))
        } else {
            Err(input)
        }
    }
}

impl<I, P, F> ParserMut<I> for ValidateParser<P, F>
where
    I: Input,
    P: ParserMut<I>,
    F: FnMut(&P::Output) -> bool,
{
    type Output = P::Output;

    fn parse_mut(&mut self, input: I) -> ParseResult<I, Self::Output> {
        let (remaining, parsed) = self.parser.parse_mut(input.clone())?;
        if (self.func)(&parsed) {
            Ok((remaining, parsed))
        } else {
            Err(input)
        }
    }
}

impl<I, P, F> ParserOnce<I> for ValidateParser<P, F>
where
    I: Input,
    P: ParserOnce<I>,
    F: FnOnce(&P::Output) -> bool,
{
    type Output = P::Output;

    fn parse_once(self, input: I) -> ParseResult<I, Self::Output> {
        let (remaining, parsed) = self.parser.parse_once(input.clone())?;
        if (self.func)(&parsed) {
            Ok((remaining, parsed))
        } else {
            Err(input)
        }
    }
}
