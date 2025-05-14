use super::{Parser, ParserMut, ParserOnce};
use crate::parse::input::{Input, ParseResult};

macro_rules! typed_crash {
    ($type: ty) => {
        crate::parse::parser::typed_crash_parser::<$type>()
    };
}
pub(super) use typed_crash;

pub fn typed_crash_parser<O>() -> CrashParser<O> {
    CrashParser::new()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CrashParser<O> {
    _phantom: std::marker::PhantomData<O>,
}

impl<O> CrashParser<O> {
    fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<I, O> Parser<I> for CrashParser<O>
where
    I: Input,
{
    type Output = O;

    fn parse(&self, _: I) -> ParseResult<I, Self::Output> {
        panic!("crash parser invoked!");
    }
}

impl<I, O> ParserMut<I> for CrashParser<O>
where
    I: Input,
{
    type Output = O;

    fn parse_mut(&mut self, input: I) -> ParseResult<I, Self::Output> {
        self.parse(input)
    }
}

impl<I, O> ParserOnce<I> for CrashParser<O>
where
    I: Input,
{
    type Output = O;

    fn parse_once(self, input: I) -> ParseResult<I, Self::Output> {
        self.parse(input)
    }
}

macro_rules! typed_fail {
    ($type: ty) => {
        crate::parse::parser::typed_fail_parser::<$type>()
    };
}
pub(super) use typed_fail;

/// A utility parser that always fails.
///
/// Mostly useful for testing purposes.
pub fn typed_fail_parser<O>() -> FailParser<O> {
    FailParser::new()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FailParser<O> {
    _phantom: std::marker::PhantomData<O>,
}

impl<O> FailParser<O> {
    fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<I, O> Parser<I> for FailParser<O>
where
    I: Input,
{
    type Output = O;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output> {
        Err(input)
    }
}

impl<I, O> ParserMut<I> for FailParser<O>
where
    I: Input,
{
    type Output = O;

    fn parse_mut(&mut self, input: I) -> ParseResult<I, Self::Output> {
        Err(input)
    }
}

impl<I, O> ParserOnce<I> for FailParser<O>
where
    I: Input,
{
    type Output = O;

    fn parse_once(self, input: I) -> ParseResult<I, Self::Output> {
        Err(input)
    }
}
