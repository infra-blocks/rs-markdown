use super::{Parser, ParserMut, ParserOnce};
use crate::parse::input::{Input, ParseResult};

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
    I: Input,
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

impl<I, P> ParserMut<I> for ZeroToManyParser<P>
where
    I: Input,
    P: ParserMut<I>,
{
    type Output = Vec<P::Output>;

    fn parse_mut(&mut self, input: I) -> ParseResult<I, Self::Output> {
        let mut remaining = input;
        let mut results = Vec::new();

        let remaining = loop {
            match self.parser.parse_mut(remaining) {
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

// Only available if the parser can clone itself.
impl<I, P> ParserOnce<I> for ZeroToManyParser<P>
where
    I: Input,
    P: ParserOnce<I> + Clone,
{
    type Output = Vec<P::Output>;

    fn parse_once(self, input: I) -> ParseResult<I, Self::Output> {
        let mut remaining = input;
        let mut results = Vec::new();

        let remaining = loop {
            match self.parser.clone().parse_once(remaining) {
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
    use crate::parse::input::ParseQuantity;

    fn parse_success<I: Input<Item = &'static str>>(input: I) -> ParseResult<I, &'static str> {
        let segment = input.segment();
        if segment.len() < 4 {
            return input.failed();
        }
        input.parsed(ParseQuantity::Bytes(4), &segment[..4])
    }

    fn parse_failure<I: Input>(input: I) -> ParseResult<I, &'static str> {
        input.failed()
    }

    #[test]
    fn test_should_return_empty_array_on_failure() {
        let parser = parse_failure.zero_to_many();
        let result = parser.parse("test1234");
        assert_eq!(result, Ok(("test1234", vec![])));
    }

    #[test]
    fn test_should_return_as_many_values_as_possible() {
        let parser = parse_success.zero_to_many();
        let result = parser.parse("test1234");
        assert_eq!(result, Ok(("", vec!["test", "1234"])));
    }

    #[test]
    fn test_should_support_parser_mut() {
        let mut x = 0;
        let parser = |input: &'static str| {
            x += 1;
            let segment = input;
            if input.len() < 4 {
                return input.failed();
            }
            input.parsed(ParseQuantity::Bytes(4), &segment[..4])
        };
        let mut parser = parser.zero_to_many();
        let result = parser.parse_mut("test123456");
        assert_eq!(result, Ok(("56", vec!["test", "1234"])));
    }

    #[test]
    fn test_should_support_parser_once() {
        let result = vec!["test"];
        let parser = |input: &'static str| {
            if input.len() < 4 {
                return input.failed();
            }
            input.parsed(ParseQuantity::Bytes(4), result)
        };
        let parser = parser.zero_to_many();
        let result = parser.parse_once("test123456");
        assert_eq!(result, Ok(("56", vec![vec!["test"], vec!["test"]])));
    }
}
