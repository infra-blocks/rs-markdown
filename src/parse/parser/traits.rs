use crate::parse::input::{Input, ParseResult};

pub trait Parser<I>
where
    I: Input,
{
    type Output;
    fn parse(&self, input: I) -> ParseResult<I, Self::Output>;
}

impl<I, O, T> Parser<I> for T
where
    I: Input,
    T: Fn(I) -> ParseResult<I, O>,
{
    type Output = O;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output> {
        self(input)
    }
}

pub trait ParserMut<I>
where
    I: Input,
{
    type Output;

    fn parse_mut(&mut self, input: I) -> ParseResult<I, Self::Output>;
}

impl<I, O, T> ParserMut<I> for T
where
    I: Input,
    T: FnMut(I) -> ParseResult<I, O>,
{
    type Output = O;

    fn parse_mut(&mut self, input: I) -> ParseResult<I, Self::Output> {
        self(input)
    }
}

pub trait ParserOnce<I>
where
    I: Input,
{
    type Output;

    fn parse_once(self, input: I) -> ParseResult<I, Self::Output>;
}

impl<I, O, T> ParserOnce<I> for T
where
    I: Input,
    T: FnOnce(I) -> ParseResult<I, O>,
{
    type Output = O;

    fn parse_once(self, input: I) -> ParseResult<I, Self::Output> {
        self(input)
    }
}
