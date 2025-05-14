use super::{Parser, ParserMut, ParserOnce};
use crate::parse::input::{Input, ParseResult};

pub trait And<R>: Sized {
    fn and(self, right: R) -> AndParser<Self, R>;
}

impl<R, T> And<R> for T {
    fn and(self, right: R) -> AndParser<Self, R> {
        and(self, right)
    }
}

pub fn and<L, R>(left: L, right: R) -> AndParser<L, R> {
    AndParser::new(left, right)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AndParser<L, R> {
    left: L,
    right: R,
}

impl<L, R> AndParser<L, R> {
    fn new(left: L, right: R) -> Self {
        Self { left, right }
    }
}

impl<I, L, R> Parser<I> for AndParser<L, R>
where
    I: Input + Clone,
    L: Parser<I>,
    R: Parser<I>,
{
    type Output = (L::Output, R::Output);

    fn parse(&self, input: I) -> ParseResult<I, Self::Output> {
        let (remaining, left) = self.left.parse(input.clone())?;
        match self.right.parse(remaining) {
            Ok((remaining, right)) => Ok((remaining, (left, right))),
            // An error invalidates the whole parser and we rewind from the beginning.
            Err(_) => Err(input),
        }
    }
}

impl<I, L, R> ParserMut<I> for AndParser<L, R>
where
    I: Input,
    L: ParserMut<I>,
    R: ParserMut<I>,
{
    type Output = (L::Output, R::Output);

    fn parse_mut(&mut self, input: I) -> ParseResult<I, Self::Output> {
        let (remaining, left) = self.left.parse_mut(input.clone())?;
        match self.right.parse_mut(remaining) {
            Ok((remaining, right)) => Ok((remaining, (left, right))),
            // An error invalidates the whole parser and we rewind from the beginning.
            Err(_) => Err(input),
        }
    }
}

impl<I, L, R> ParserOnce<I> for AndParser<L, R>
where
    I: Input,
    L: ParserOnce<I>,
    R: ParserOnce<I>,
{
    type Output = (L::Output, R::Output);

    fn parse_once(self, input: I) -> ParseResult<I, Self::Output> {
        let (remaining, left) = self.left.parse_once(input.clone())?;
        match self.right.parse_once(remaining) {
            Ok((remaining, right)) => Ok((remaining, (left, right))),
            // An error invalidates the whole parser and we rewind from the beginning.
            Err(_) => Err(input),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parse::input::ParseQuantity;
    use std::vec;

    // The test parsing function that will succeed. The input needs to be at least 4 bytes long,
    // because it simply parses the first 4 bytes and returns them as output.
    fn parse_success<I: Input<Item = &'static str>>(input: I) -> ParseResult<I, &'static str> {
        let segment = input.segment();
        input.parsed(ParseQuantity::Bytes(4), &segment[..4])
    }

    // The test parsing function that will fail. It simply returns the input without parsing it.
    fn parse_failure<I: Input>(input: I) -> ParseResult<I, &'static str> {
        input.failed()
    }

    #[test]
    fn test_rejects_when_left_rejects() {
        let parser = parse_failure.and(parse_success);
        let result = parser.parse("test");
        assert_eq!(result, Err("test"));
    }

    #[test]
    fn test_rejects_when_right_rejects() {
        let parser = parse_failure.and(parse_failure);
        let result = parser.parse("test");
        // Even though the first parser succeeds and consumes the first 4 bytes, a failure in the
        // second should rewind the whole thang.
        assert_eq!(result, Err("test"));
    }

    #[test]
    fn test_success_when_both_succeed() {
        let parser = parse_success.and(parse_success);
        let result = parser.parse("test1234");
        assert_eq!(result, Ok(("", ("test", "1234"))));
    }

    #[test]
    fn test_parse_mut_when_parsers_are_mut() {
        let mut x = 0;
        let left = |input: &'static str| {
            x += 1;
            let segment = input;
            input.parsed(ParseQuantity::Bytes(4), &segment[..4])
        };
        let mut parser = left.and(parse_success);
        let result = parser.parse_mut("test1234");
        assert_eq!(result, Ok(("", ("test", "1234"))));
    }

    #[test]
    fn test_parse_once_when_parsers_are_once() {
        let result = vec!["test"];
        let left = |input: &'static str| input.parsed(ParseQuantity::Bytes(4), result);
        let parser = left.and(parse_success);
        let result = parser.parse_once("test1234");
        assert_eq!(result, Ok(("", (vec!["test"], "1234"))));
    }
}
