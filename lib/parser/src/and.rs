use crate::{ParseResult, Parser};

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
    I: Clone,
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::{take, typed_fail, utils::alias};

    alias!(fail, typed_fail![&'static str]);

    #[test]
    fn test_rejects_when_left_rejects() {
        let parser = and(fail!(), take(4));
        let result = parser.parse("test");
        assert_eq!(Err("test"), result);
    }

    #[test]
    fn test_rejects_when_right_rejects() {
        let parser = take(4).and(fail!());
        let result = parser.parse("test");
        // Even though the first parser succeeds and consumes the first 4 bytes, a failure in the
        // second should rewind the whole thang.
        assert_eq!(Err("test"), result);
    }

    #[test]
    fn test_success_when_both_succeed() {
        let parser = take(4).and(take(4));
        let result = parser.parse("test1234");
        assert_eq!(Ok(("", ("test", "1234"))), result);
    }
}
