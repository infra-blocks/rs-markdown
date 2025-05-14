use super::{Or, Parser, ParserMut, ParserOnce};
use crate::parse::input::{Input, ParseResult};

pub fn one_of<L>(parsers: L) -> OneOfParser<L> {
    OneOfParser::new(parsers)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OneOfParser<L> {
    parsers: L,
}

impl<L> OneOfParser<L> {
    fn new(parsers: L) -> Self {
        Self { parsers }
    }
}

impl<I, T1> Parser<I> for OneOfParser<(T1,)>
where
    I: Input,
    T1: Parser<I>,
{
    type Output = T1::Output;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output> {
        self.parsers.0.parse(input)
    }
}

impl<I, T1, T2> Parser<I> for OneOfParser<(T1, T2)>
where
    I: Input,
    T1: Parser<I>,
    T2: Parser<I, Output = T1::Output>,
{
    type Output = T1::Output;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output> {
        let left = |input: I| self.parsers.0.parse(input);
        let right = |input: I| self.parsers.1.parse(input);
        left.or(right).parse(input)
    }
}

impl<I, T1, T2, T3> Parser<I> for OneOfParser<(T1, T2, T3)>
where
    I: Input,
    T1: Parser<I>,
    T2: Parser<I, Output = T1::Output>,
    T3: Parser<I, Output = T1::Output>,
{
    type Output = T1::Output;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output> {
        let first = |input: I| self.parsers.0.parse(input);
        let second = |input: I| self.parsers.1.parse(input);
        let third = |input: I| self.parsers.2.parse(input);
        first.or(second).or(third).parse(input)
    }
}

impl<I, T1, T2, T3, T4> Parser<I> for OneOfParser<(T1, T2, T3, T4)>
where
    I: Input,
    T1: Parser<I>,
    T2: Parser<I, Output = T1::Output>,
    T3: Parser<I, Output = T1::Output>,
    T4: Parser<I, Output = T1::Output>,
{
    type Output = T1::Output;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output> {
        let first = |input: I| self.parsers.0.parse(input);
        let second = |input: I| self.parsers.1.parse(input);
        let third = |input: I| self.parsers.2.parse(input);
        let fourth = |input: I| self.parsers.3.parse(input);
        first.or(second).or(third).or(fourth).parse(input)
    }
}

impl<I, T1, T2, T3, T4, T5> Parser<I> for OneOfParser<(T1, T2, T3, T4, T5)>
where
    I: Input,
    T1: Parser<I>,
    T2: Parser<I, Output = T1::Output>,
    T3: Parser<I, Output = T1::Output>,
    T4: Parser<I, Output = T1::Output>,
    T5: Parser<I, Output = T1::Output>,
{
    type Output = T1::Output;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output> {
        let first = |input: I| self.parsers.0.parse(input);
        let second = |input: I| self.parsers.1.parse(input);
        let third = |input: I| self.parsers.2.parse(input);
        let fourth = |input: I| self.parsers.3.parse(input);
        let fifth = |input: I| self.parsers.4.parse(input);
        first.or(second).or(third).or(fourth).or(fifth).parse(input)
    }
}

impl<I, T1> ParserMut<I> for OneOfParser<(T1,)>
where
    I: Input,
    T1: ParserMut<I>,
{
    type Output = T1::Output;

    fn parse_mut(&mut self, input: I) -> ParseResult<I, Self::Output> {
        self.parsers.0.parse_mut(input)
    }
}

impl<I, T1, T2> ParserMut<I> for OneOfParser<(T1, T2)>
where
    I: Input,
    T1: ParserMut<I>,
    T2: ParserMut<I, Output = T1::Output>,
{
    type Output = T1::Output;

    fn parse_mut(&mut self, input: I) -> ParseResult<I, Self::Output> {
        let left = |input: I| self.parsers.0.parse_mut(input);
        let right = |input: I| self.parsers.1.parse_mut(input);
        left.or(right).parse_mut(input)
    }
}

impl<I, T1, T2, T3> ParserMut<I> for OneOfParser<(T1, T2, T3)>
where
    I: Input,
    T1: ParserMut<I>,
    T2: ParserMut<I, Output = T1::Output>,
    T3: ParserMut<I, Output = T1::Output>,
{
    type Output = T1::Output;

    fn parse_mut(&mut self, input: I) -> ParseResult<I, Self::Output> {
        let first = |input: I| self.parsers.0.parse_mut(input);
        let second = |input: I| self.parsers.1.parse_mut(input);
        let third = |input: I| self.parsers.2.parse_mut(input);
        first.or(second).or(third).parse_mut(input)
    }
}

impl<I, T1, T2, T3, T4> ParserMut<I> for OneOfParser<(T1, T2, T3, T4)>
where
    I: Input,
    T1: ParserMut<I>,
    T2: ParserMut<I, Output = T1::Output>,
    T3: ParserMut<I, Output = T1::Output>,
    T4: ParserMut<I, Output = T1::Output>,
{
    type Output = T1::Output;

    fn parse_mut(&mut self, input: I) -> ParseResult<I, Self::Output> {
        let first = |input: I| self.parsers.0.parse_mut(input);
        let second = |input: I| self.parsers.1.parse_mut(input);
        let third = |input: I| self.parsers.2.parse_mut(input);
        let fourth = |input: I| self.parsers.3.parse_mut(input);
        first.or(second).or(third).or(fourth).parse_mut(input)
    }
}

impl<I, T1, T2, T3, T4, T5> ParserMut<I> for OneOfParser<(T1, T2, T3, T4, T5)>
where
    I: Input,
    T1: ParserMut<I>,
    T2: ParserMut<I, Output = T1::Output>,
    T3: ParserMut<I, Output = T1::Output>,
    T4: ParserMut<I, Output = T1::Output>,
    T5: ParserMut<I, Output = T1::Output>,
{
    type Output = T1::Output;

    fn parse_mut(&mut self, input: I) -> ParseResult<I, Self::Output> {
        let first = |input: I| self.parsers.0.parse_mut(input);
        let second = |input: I| self.parsers.1.parse_mut(input);
        let third = |input: I| self.parsers.2.parse_mut(input);
        let fourth = |input: I| self.parsers.3.parse_mut(input);
        let fifth = |input: I| self.parsers.4.parse_mut(input);
        first
            .or(second)
            .or(third)
            .or(fourth)
            .or(fifth)
            .parse_mut(input)
    }
}

impl<I, T1> ParserOnce<I> for OneOfParser<(T1,)>
where
    I: Input,
    T1: ParserOnce<I>,
{
    type Output = T1::Output;

    fn parse_once(self, input: I) -> ParseResult<I, Self::Output> {
        self.parsers.0.parse_once(input)
    }
}

impl<I, T1, T2> ParserOnce<I> for OneOfParser<(T1, T2)>
where
    I: Input,
    T1: ParserOnce<I>,
    T2: ParserOnce<I, Output = T1::Output>,
{
    type Output = T1::Output;

    fn parse_once(self, input: I) -> ParseResult<I, Self::Output> {
        let left = |input: I| self.parsers.0.parse_once(input);
        let right = |input: I| self.parsers.1.parse_once(input);
        left.or(right).parse_once(input)
    }
}

impl<I, T1, T2, T3> ParserOnce<I> for OneOfParser<(T1, T2, T3)>
where
    I: Input,
    T1: ParserOnce<I>,
    T2: ParserOnce<I, Output = T1::Output>,
    T3: ParserOnce<I, Output = T1::Output>,
{
    type Output = T1::Output;

    fn parse_once(self, input: I) -> ParseResult<I, Self::Output> {
        let first = |input: I| self.parsers.0.parse_once(input);
        let second = |input: I| self.parsers.1.parse_once(input);
        let third = |input: I| self.parsers.2.parse_once(input);
        first.or(second).or(third).parse_once(input)
    }
}

impl<I, T1, T2, T3, T4> ParserOnce<I> for OneOfParser<(T1, T2, T3, T4)>
where
    I: Input,
    T1: ParserOnce<I>,
    T2: ParserOnce<I, Output = T1::Output>,
    T3: ParserOnce<I, Output = T1::Output>,
    T4: ParserOnce<I, Output = T1::Output>,
{
    type Output = T1::Output;

    fn parse_once(self, input: I) -> ParseResult<I, Self::Output> {
        let first = |input: I| self.parsers.0.parse_once(input);
        let second = |input: I| self.parsers.1.parse_once(input);
        let third = |input: I| self.parsers.2.parse_once(input);
        let fourth = |input: I| self.parsers.3.parse_once(input);
        first.or(second).or(third).or(fourth).parse_once(input)
    }
}

impl<I, T1, T2, T3, T4, T5> ParserOnce<I> for OneOfParser<(T1, T2, T3, T4, T5)>
where
    I: Input,
    T1: ParserOnce<I>,
    T2: ParserOnce<I, Output = T1::Output>,
    T3: ParserOnce<I, Output = T1::Output>,
    T4: ParserOnce<I, Output = T1::Output>,
    T5: ParserOnce<I, Output = T1::Output>,
{
    type Output = T1::Output;

    fn parse_once(self, input: I) -> ParseResult<I, Self::Output> {
        let first = |input: I| self.parsers.0.parse_once(input);
        let second = |input: I| self.parsers.1.parse_once(input);
        let third = |input: I| self.parsers.2.parse_once(input);
        let fourth = |input: I| self.parsers.3.parse_once(input);
        let fifth = |input: I| self.parsers.4.parse_once(input);
        first
            .or(second)
            .or(third)
            .or(fourth)
            .or(fifth)
            .parse_once(input)
    }
}
