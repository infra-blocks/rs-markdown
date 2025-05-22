use super::Parser;
use crate::{And, AndParser, Map, ParseResult};

pub trait Preceded<R>: Sized {
    fn preceded(self, parser: R) -> PrecededParser<Self, R>;
}

impl<R, T> Preceded<R> for T {
    fn preceded(self, parser: R) -> PrecededParser<Self, R> {
        preceded(self, parser)
    }
}

pub fn preceded<L, R>(preceding: L, after: R) -> PrecededParser<L, R> {
    PrecededParser::new(preceding.and(after))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrecededParser<L, R> {
    parser: AndParser<L, R>,
}

impl<L, R> PrecededParser<L, R> {
    fn new(parser: AndParser<L, R>) -> Self {
        Self { parser }
    }
}

impl<I, L, R> Parser<I> for PrecededParser<L, R>
where
    I: Clone,
    L: Parser<I>,
    R: Parser<I>,
{
    type Output = R::Output;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output> {
        let parser = |input| self.parser.parse(input);
        parser.map(|(_, r)| r).parse(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{take, typed_fail, utils::alias};

    alias!(fail, typed_fail!(&'static str));

    #[test]
    fn test_rejects_when_left_rejects() {
        let parser = preceded(fail!(), take(4));
        let result = parser.parse("test");
        assert_eq!(Err("test"), result);
    }

    #[test]
    fn test_rejects_when_right_rejects() {
        let parser = take(4).preceded(fail!());
        let result = parser.parse("test");
        // Even though the first parser succeeds and consumes the first 4 bytes, a failure in the
        // second should rewind the whole thang, just like an `and` parser.
        assert_eq!(Err("test"), result);
    }

    #[test]
    fn test_success_when_both_succeed() {
        let parser = preceded(take(4), take(4));
        let result = parser.parse("test1234");
        assert_eq!(Ok(("", "1234")), result);
    }
}
