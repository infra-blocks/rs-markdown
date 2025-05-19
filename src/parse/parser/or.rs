use super::{Parser, ParserMut, ParserOnce};
use crate::parse::{ParseResult, input::Input};

pub trait Or<R>: Sized {
    fn or(self, right: R) -> OrParser<Self, R>;
}

impl<R, T> Or<R> for T {
    fn or(self, right: R) -> OrParser<Self, R> {
        or(self, right)
    }
}

pub fn or<L, R>(left: L, right: R) -> OrParser<L, R> {
    OrParser::new(left, right)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrParser<L, R> {
    left: L,
    right: R,
}

impl<L, R> OrParser<L, R> {
    fn new(left: L, right: R) -> Self {
        Self { left, right }
    }
}

impl<I, L, R> Parser<I> for OrParser<L, R>
where
    I: Input,
    L: Parser<I>,
    R: Parser<I, Output = L::Output>,
{
    type Output = L::Output;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output> {
        match self.left.parse(input) {
            Ok((remaining, parsed)) => Ok((remaining, parsed)),
            Err(input) => self.right.parse(input),
        }
    }
}

impl<I, L, R> ParserMut<I> for OrParser<L, R>
where
    I: Input,
    L: ParserMut<I>,
    R: ParserMut<I, Output = L::Output>,
{
    type Output = L::Output;

    fn parse_mut(&mut self, input: I) -> ParseResult<I, Self::Output> {
        match self.left.parse_mut(input) {
            Ok((remaining, parsed)) => Ok((remaining, parsed)),
            Err(remaining) => self.right.parse_mut(remaining),
        }
    }
}

impl<I, L, R> ParserOnce<I> for OrParser<L, R>
where
    I: Input,
    L: ParserOnce<I>,
    R: ParserOnce<I, Output = L::Output>,
{
    type Output = L::Output;

    fn parse_once(self, input: I) -> ParseResult<I, Self::Output> {
        match self.left.parse_once(input) {
            Ok((remaining, parsed)) => Ok((remaining, parsed)),
            Err(remaining) => self.right.parse_once(remaining),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parse::{
        parser::{take_chars, typed_crash, typed_fail},
        utils::alias,
    };

    alias!(fail, typed_fail![&'static str]);
    alias!(crash, typed_crash![&'static str]);

    #[test]
    fn test_rejects_when_both_reject() {
        let parser = fail!().or(fail!());
        let result = parser.parse("test");
        assert_eq!(Err("test"), result);
    }

    #[test]
    fn test_succeeds_when_left_succeeds_and_does_not_call_right() {
        let parser = take_chars(4).or(crash!());
        let result = parser.parse("test1234");
        assert_eq!(Ok(("1234", "test")), result);
    }

    #[test]
    fn test_succeeds_when_right_succeeds() {
        let parser = fail!().or(take_chars(4));
        let result = parser.parse("test1234");
        assert_eq!(Ok(("1234", "test")), result);
    }

    #[test]
    fn test_parse_mut_when_parsers_are_mut() {
        let mut parser = take_chars(4);
        let parser = |input| parser.parse_mut(input);
        let mut parser = fail!().or(parser);
        let result = parser.parse_mut("test1234");
        assert_eq!(Ok(("1234", "test")), result);
    }

    #[test]
    fn test_parse_once_when_parsers_are_once() {
        let parser = take_chars(4);
        let parser = |input| parser.parse_once(input);
        let parser = fail!().or(parser);
        let result = parser.parse_once("test1234");
        assert_eq!(Ok(("1234", "test")), result);
    }
}
