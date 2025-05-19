use super::{Parser, ParserMut, ParserOnce};
use crate::parse::{ParseResult, input::Input};

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
    I: Input,
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

impl<I, P> ParserMut<I> for OneToManyParser<P>
where
    I: Input,
    P: ParserMut<I>,
{
    type Output = Vec<P::Output>;

    fn parse_mut(&mut self, input: I) -> ParseResult<I, Self::Output> {
        let first_result = self.parser.parse_mut(input)?;
        let mut remaining = first_result.0;
        let mut results = vec![first_result.1];
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
impl<I, P> ParserOnce<I> for OneToManyParser<P>
where
    I: Input,
    P: ParserOnce<I> + Clone,
{
    type Output = Vec<P::Output>;

    fn parse_once(self, input: I) -> ParseResult<I, Self::Output> {
        let first_result = self.parser.clone().parse_once(input)?;
        let mut remaining = first_result.0;
        let mut results = vec![first_result.1];
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
    use crate::{
        lines,
        parse::{
            parser::{take_chars, typed_fail},
            utils::alias,
        },
    };

    alias!(fail, typed_fail![&'static str]);

    #[test]
    fn test_should_fail_if_cannot_parse_one() {
        let parser = fail!().one_to_many();
        assert_eq!(Err(lines!("test1234")), parser.parse(lines!("test1234")));
    }

    #[test]
    fn test_should_succeed_if_it_can_parse_one() {
        let parser = take_chars(4).one_to_many();
        let result = parser.parse(lines!("test12"));
        assert_eq!(Ok((lines!("12"), vec!["test"])), result);
    }

    #[test]
    fn test_should_return_as_many_values_as_possible() {
        let parser = take_chars(4).one_to_many();
        let result = parser.parse(lines!("test123456"));
        assert_eq!(Ok((lines!("56"), vec!["test", "1234"])), result);
    }

    #[test]
    fn test_should_support_parser_mut() {
        let mut parser = take_chars(4);
        let parser = |input| parser.parse_mut(input);
        let mut parser = parser.one_to_many();
        let result = parser.parse_mut(lines!("test123456"));
        assert_eq!(Ok((lines!("56"), vec!["test", "1234"])), result);
    }

    #[test]
    fn test_should_support_parser_once() {
        let parser = take_chars(4);
        let parser = |input| parser.parse_once(input);
        let parser = parser.one_to_many();
        let result = parser.parse_once(lines!("test123456"));
        assert_eq!(Ok((lines!("56"), vec!["test", "1234"])), result);
    }
}
