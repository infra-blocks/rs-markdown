use super::{Or, Parser, ParserMut, ParserOnce};
use crate::parse::{ParseResult, input::Input};

pub fn one_of<L>(parsers: L) -> OneOfParser<L> {
    OneOfParser::new(parsers)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OneOfParser<L> {
    parsers: L,
}

impl<L> OneOfParser<L> {
    fn new(parsers: L) -> Self {
        Self { parsers }
    }
}

impl<I, T1, T2> Parser<I> for OneOfParser<(T1, T2)>
where
    I: Input,
    T1: Parser<I>,
    T2: Parser<I, Output = T1::Output>,
{
    type Output = T1::Output;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output> {
        let left = |input: I| self.parsers.0.parse(input);
        let right = |input: I| self.parsers.1.parse(input);
        left.or(right).parse(input)
    }
}

impl<I, T1, T2, T3> Parser<I> for OneOfParser<(T1, T2, T3)>
where
    I: Input,
    T1: Parser<I>,
    T2: Parser<I, Output = T1::Output>,
    T3: Parser<I, Output = T1::Output>,
{
    type Output = T1::Output;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output> {
        let first = |input: I| self.parsers.0.parse(input);
        let second = |input: I| self.parsers.1.parse(input);
        let third = |input: I| self.parsers.2.parse(input);
        first.or(second).or(third).parse(input)
    }
}

impl<I, T1, T2, T3, T4> Parser<I> for OneOfParser<(T1, T2, T3, T4)>
where
    I: Input,
    T1: Parser<I>,
    T2: Parser<I, Output = T1::Output>,
    T3: Parser<I, Output = T1::Output>,
    T4: Parser<I, Output = T1::Output>,
{
    type Output = T1::Output;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output> {
        let first = |input: I| self.parsers.0.parse(input);
        let second = |input: I| self.parsers.1.parse(input);
        let third = |input: I| self.parsers.2.parse(input);
        let fourth = |input: I| self.parsers.3.parse(input);
        first.or(second).or(third).or(fourth).parse(input)
    }
}

impl<I, T1, T2, T3, T4, T5> Parser<I> for OneOfParser<(T1, T2, T3, T4, T5)>
where
    I: Input,
    T1: Parser<I>,
    T2: Parser<I, Output = T1::Output>,
    T3: Parser<I, Output = T1::Output>,
    T4: Parser<I, Output = T1::Output>,
    T5: Parser<I, Output = T1::Output>,
{
    type Output = T1::Output;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output> {
        let first = |input: I| self.parsers.0.parse(input);
        let second = |input: I| self.parsers.1.parse(input);
        let third = |input: I| self.parsers.2.parse(input);
        let fourth = |input: I| self.parsers.3.parse(input);
        let fifth = |input: I| self.parsers.4.parse(input);
        first.or(second).or(third).or(fourth).or(fifth).parse(input)
    }
}

impl<I, T1, T2> ParserMut<I> for OneOfParser<(T1, T2)>
where
    I: Input,
    T1: ParserMut<I>,
    T2: ParserMut<I, Output = T1::Output>,
{
    type Output = T1::Output;

    fn parse_mut(&mut self, input: I) -> ParseResult<I, Self::Output> {
        let left = |input: I| self.parsers.0.parse_mut(input);
        let right = |input: I| self.parsers.1.parse_mut(input);
        left.or(right).parse_mut(input)
    }
}

impl<I, T1, T2, T3> ParserMut<I> for OneOfParser<(T1, T2, T3)>
where
    I: Input,
    T1: ParserMut<I>,
    T2: ParserMut<I, Output = T1::Output>,
    T3: ParserMut<I, Output = T1::Output>,
{
    type Output = T1::Output;

    fn parse_mut(&mut self, input: I) -> ParseResult<I, Self::Output> {
        let first = |input: I| self.parsers.0.parse_mut(input);
        let second = |input: I| self.parsers.1.parse_mut(input);
        let third = |input: I| self.parsers.2.parse_mut(input);
        first.or(second).or(third).parse_mut(input)
    }
}

impl<I, T1, T2, T3, T4> ParserMut<I> for OneOfParser<(T1, T2, T3, T4)>
where
    I: Input,
    T1: ParserMut<I>,
    T2: ParserMut<I, Output = T1::Output>,
    T3: ParserMut<I, Output = T1::Output>,
    T4: ParserMut<I, Output = T1::Output>,
{
    type Output = T1::Output;

    fn parse_mut(&mut self, input: I) -> ParseResult<I, Self::Output> {
        let first = |input: I| self.parsers.0.parse_mut(input);
        let second = |input: I| self.parsers.1.parse_mut(input);
        let third = |input: I| self.parsers.2.parse_mut(input);
        let fourth = |input: I| self.parsers.3.parse_mut(input);
        first.or(second).or(third).or(fourth).parse_mut(input)
    }
}

impl<I, T1, T2, T3, T4, T5> ParserMut<I> for OneOfParser<(T1, T2, T3, T4, T5)>
where
    I: Input,
    T1: ParserMut<I>,
    T2: ParserMut<I, Output = T1::Output>,
    T3: ParserMut<I, Output = T1::Output>,
    T4: ParserMut<I, Output = T1::Output>,
    T5: ParserMut<I, Output = T1::Output>,
{
    type Output = T1::Output;

    fn parse_mut(&mut self, input: I) -> ParseResult<I, Self::Output> {
        let first = |input: I| self.parsers.0.parse_mut(input);
        let second = |input: I| self.parsers.1.parse_mut(input);
        let third = |input: I| self.parsers.2.parse_mut(input);
        let fourth = |input: I| self.parsers.3.parse_mut(input);
        let fifth = |input: I| self.parsers.4.parse_mut(input);
        first
            .or(second)
            .or(third)
            .or(fourth)
            .or(fifth)
            .parse_mut(input)
    }
}

impl<I, T1, T2> ParserOnce<I> for OneOfParser<(T1, T2)>
where
    I: Input,
    T1: ParserOnce<I>,
    T2: ParserOnce<I, Output = T1::Output>,
{
    type Output = T1::Output;

    fn parse_once(self, input: I) -> ParseResult<I, Self::Output> {
        let left = |input: I| self.parsers.0.parse_once(input);
        let right = |input: I| self.parsers.1.parse_once(input);
        left.or(right).parse_once(input)
    }
}

impl<I, T1, T2, T3> ParserOnce<I> for OneOfParser<(T1, T2, T3)>
where
    I: Input,
    T1: ParserOnce<I>,
    T2: ParserOnce<I, Output = T1::Output>,
    T3: ParserOnce<I, Output = T1::Output>,
{
    type Output = T1::Output;

    fn parse_once(self, input: I) -> ParseResult<I, Self::Output> {
        let first = |input: I| self.parsers.0.parse_once(input);
        let second = |input: I| self.parsers.1.parse_once(input);
        let third = |input: I| self.parsers.2.parse_once(input);
        first.or(second).or(third).parse_once(input)
    }
}

impl<I, T1, T2, T3, T4> ParserOnce<I> for OneOfParser<(T1, T2, T3, T4)>
where
    I: Input,
    T1: ParserOnce<I>,
    T2: ParserOnce<I, Output = T1::Output>,
    T3: ParserOnce<I, Output = T1::Output>,
    T4: ParserOnce<I, Output = T1::Output>,
{
    type Output = T1::Output;

    fn parse_once(self, input: I) -> ParseResult<I, Self::Output> {
        let first = |input: I| self.parsers.0.parse_once(input);
        let second = |input: I| self.parsers.1.parse_once(input);
        let third = |input: I| self.parsers.2.parse_once(input);
        let fourth = |input: I| self.parsers.3.parse_once(input);
        first.or(second).or(third).or(fourth).parse_once(input)
    }
}

impl<I, T1, T2, T3, T4, T5> ParserOnce<I> for OneOfParser<(T1, T2, T3, T4, T5)>
where
    I: Input,
    T1: ParserOnce<I>,
    T2: ParserOnce<I, Output = T1::Output>,
    T3: ParserOnce<I, Output = T1::Output>,
    T4: ParserOnce<I, Output = T1::Output>,
    T5: ParserOnce<I, Output = T1::Output>,
{
    type Output = T1::Output;

    fn parse_once(self, input: I) -> ParseResult<I, Self::Output> {
        let first = |input: I| self.parsers.0.parse_once(input);
        let second = |input: I| self.parsers.1.parse_once(input);
        let third = |input: I| self.parsers.2.parse_once(input);
        let fourth = |input: I| self.parsers.3.parse_once(input);
        let fifth = |input: I| self.parsers.4.parse_once(input);
        first
            .or(second)
            .or(third)
            .or(fourth)
            .or(fifth)
            .parse_once(input)
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
    fn should_fail_if_all_parsers_fail() {
        let parser = one_of((fail!(), fail!()));
        assert_eq!(Err("test1234"), parser.parse("test1234"));
    }

    #[test]
    fn should_succeed_and_not_call_other_parsers_when_first_succeeds() {
        let parser = one_of((take_chars(4), crash!()));
        let result = parser.parse("test1234");
        assert_eq!(Ok(("1234", "test")), result);
    }

    #[test]
    fn should_succeed_if_first_parser_fails_and_second_succeeds() {
        let parser = one_of((fail!(), take_chars(4)));
        let result = parser.parse("test1234");
        assert_eq!(Ok(("1234", "test")), result);
    }

    #[test]
    fn should_work_with_3_parsers() {
        let parser = one_of((fail!(), fail!(), take_chars(4)));
        let result = parser.parse("test1234");
        assert_eq!(Ok(("1234", "test")), result);
    }

    #[test]
    fn should_work_with_4_parsers() {
        let parser = one_of((fail!(), fail!(), fail!(), take_chars(4)));
        let result = parser.parse("test1234");
        assert_eq!(Ok(("1234", "test")), result);
    }

    #[test]
    fn should_parse_mut_with_parsers_mut() {
        let mut left = take_chars(4);
        let left = |input| left.parse_mut(input);
        let mut parser = one_of((left, take_chars(4)));
        let result = parser.parse_mut("test1234");
        assert_eq!(Ok(("1234", "test")), result);
    }

    #[test]
    fn should_parse_once_with_parsers_once() {
        let left = take_chars(4);
        let left = |input| left.parse_once(input);
        let parser = one_of((left, take_chars(4)));
        let result = parser.parse_once("test1234");
        assert_eq!(Ok(("1234", "test")), result);
    }
}
