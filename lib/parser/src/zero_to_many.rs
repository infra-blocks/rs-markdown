use crate::ParseResult;
use crate::Parser;

pub trait ZeroToMany: Sized {
    fn zero_to_many(self) -> ZeroToManyParser<Self>;
}

impl<T> ZeroToMany for T {
    fn zero_to_many(self) -> ZeroToManyParser<Self> {
        zero_to_many(self)
    }
}

pub fn zero_to_many<T>(parser: T) -> ZeroToManyParser<T> {
    ZeroToManyParser::new(parser)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZeroToManyParser<P> {
    parser: P,
}

impl<P> ZeroToManyParser<P> {
    fn new(parser: P) -> Self {
        Self { parser }
    }
}

impl<I, P> Parser<I> for ZeroToManyParser<P>
where
    P: Parser<I>,
{
    type Output = Vec<P::Output>;

    fn parse(&self, input: I) -> ParseResult<I, Vec<P::Output>> {
        let mut remaining = input;
        let mut results = Vec::new();

        let remaining = loop {
            match self.parser.parse(remaining) {
                Ok((next_remaining, parsed)) => {
                    results.push(parsed);
                    remaining = next_remaining;
                }
                Err(remaining) => break remaining,
            }
        };

        Ok((remaining, results))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{take, typed_fail, utils::alias};

    alias!(fail, typed_fail!(&'static str));

    #[test]
    fn test_should_return_empty_array_on_failure() {
        let parser = fail!().zero_to_many();
        let result = parser.parse("test1234");
        assert_eq!(Ok(("test1234", vec![])), result);
    }

    #[test]
    fn test_should_succeed_if_it_can_parse_one() {
        let parser = take(4).zero_to_many();
        let result = parser.parse("test12");
        assert_eq!(Ok(("12", vec!["test"])), result);
    }

    #[test]
    fn test_should_return_as_many_values_as_possible() {
        let parser = take(4).zero_to_many();
        let result = parser.parse("test123456");
        assert_eq!(Ok(("56", vec!["test", "1234"])), result);
    }
}
