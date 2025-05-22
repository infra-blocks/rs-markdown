use crate::ParseResult;
use crate::Parser;

macro_rules! typed_crash {
    ($type: ty) => {
        crate::typed_crash_parser::<$type>()
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

impl<I, O> Parser<I> for CrashParser<O> {
    type Output = O;

    fn parse(&self, _: I) -> ParseResult<I, Self::Output> {
        panic!("crash parser invoked!");
    }
}

macro_rules! typed_fail {
    ($type: ty) => {
        crate::typed_fail_parser::<$type>()
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

impl<I, O> Parser<I> for FailParser<O> {
    type Output = O;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output> {
        Err(input)
    }
}
