use super::{Parser, ParserMut, ParserOnce};
use crate::parse::input::{Input, ParseResult};

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
    use crate::parse::input::ParseQuantity;

    fn parse_success<I: Input<Item = &'static str>>(input: I) -> ParseResult<I, &'static str> {
        let segment = input.segment();
        input.parsed(ParseQuantity::Bytes(4), &segment[..4])
    }

    fn parse_failure<I: Input>(input: I) -> ParseResult<I, &'static str> {
        input.failed()
    }

    #[test]
    fn test_rejects_when_both_reject() {
        let parser = parse_failure.or(parse_failure);
        let result = parser.parse("test");
        assert_eq!(result, Err("test"));
    }

    #[test]
    fn test_success_when_left_succeeds() {
        let parser = parse_success.or(parse_failure);
        let result = parser.parse("test1234");
        assert_eq!(result, Ok(("1234", "test")));
    }

    #[test]
    fn test_success_when_right_succeeds() {
        let parser = parse_failure.or(parse_success);
        let result = parser.parse("test1234");
        assert_eq!(result, Ok(("1234", "test")));
    }

    #[test]
    fn test_parse_mut_when_parsers_are_mut() {
        let mut x = 0;
        let left = |input: &'static str| {
            x += 1;
            let segment = input;
            input.parsed(ParseQuantity::Bytes(4), &segment[..4])
        };
        let mut parser = left.or(parse_failure);
        let result = parser.parse_mut("test1234");
        assert_eq!(result, Ok(("1234", "test")));
    }

    #[test]
    fn test_parse_once_when_parsers_are_once() {
        let left_result = vec!["test"];
        // Both parsers need to have the same output type, so we make two parsers that return vecs by move.
        let left = |input: &'static str| input.parsed(ParseQuantity::Bytes(4), left_result);
        let right_result = vec!["1234"];
        let right = |input: &'static str| input.parsed(ParseQuantity::Bytes(4), right_result);
        let parser = left.or(right);
        let result = parser.parse_once("test1234");
        assert_eq!(result, Ok(("1234", vec!["test"])));
    }
}
