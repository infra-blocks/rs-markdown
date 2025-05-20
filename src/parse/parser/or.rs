use super::Parser;
use crate::parse::parser::ParseResult;

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

#[cfg(test)]
mod test {
    use super::*;
    use crate::parse::{
        parser::{take, typed_crash, typed_fail},
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
        let parser = take(4).or(crash!());
        let result = parser.parse("test1234");
        assert_eq!(Ok(("1234", "test")), result);
    }

    #[test]
    fn test_succeeds_when_right_succeeds() {
        let parser = fail!().or(take(4));
        let result = parser.parse("test1234");
        assert_eq!(Ok(("1234", "test")), result);
    }
}
