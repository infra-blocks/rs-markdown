use crate::ParseResult;
use crate::Parser;

pub trait OneToMany: Sized {
    fn one_to_many(self) -> OneToManyParser<Self>;
}

impl<T> OneToMany for T {
    fn one_to_many(self) -> OneToManyParser<Self> {
        one_to_many(self)
    }
}

pub fn one_to_many<T>(parser: T) -> OneToManyParser<T> {
    OneToManyParser::new(parser)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OneToManyParser<P> {
    parser: P,
}

impl<P> OneToManyParser<P> {
    fn new(parser: P) -> Self {
        Self { parser }
    }
}

impl<I, P> Parser<I> for OneToManyParser<P>
where
    P: Parser<I>,
{
    type Output = Vec<P::Output>;

    fn parse(&self, input: I) -> ParseResult<I, Vec<P::Output>> {
        let first_result = self.parser.parse(input)?;
        let mut remaining = first_result.0;
        let mut results = vec![first_result.1];
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

    alias!(fail, typed_fail![&'static str]);

    #[test]
    fn test_should_fail_if_cannot_parse_one() {
        let parser = fail!().one_to_many();
        assert_eq!(Err("test1234"), parser.parse("test1234"));
    }

    #[test]
    fn test_should_succeed_if_it_can_parse_one() {
        let parser = take(4).one_to_many();
        let result = parser.parse("test12");
        assert_eq!(Ok(("12", vec!["test"])), result);
    }

    #[test]
    fn test_should_return_as_many_values_as_possible() {
        let parser = take(4).one_to_many();
        let result = parser.parse("test123456");
        assert_eq!(Ok(("56", vec!["test", "1234"])), result);
    }
}
