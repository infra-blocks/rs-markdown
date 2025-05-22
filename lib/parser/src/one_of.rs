use super::{Or, Parser};
use crate::ParseResult;

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

#[cfg(test)]
mod test {
    use super::*;
    use crate::{take, typed_crash, typed_fail, utils::alias};

    alias!(fail, typed_fail![&'static str]);
    alias!(crash, typed_crash![&'static str]);

    #[test]
    fn should_fail_if_all_parsers_fail() {
        let parser = one_of((fail!(), fail!()));
        assert_eq!(Err("test1234"), parser.parse("test1234"));
    }

    #[test]
    fn should_succeed_and_not_call_other_parsers_when_first_succeeds() {
        let parser = one_of((take(4), crash!()));
        let result = parser.parse("test1234");
        assert_eq!(Ok(("1234", "test")), result);
    }

    #[test]
    fn should_succeed_if_first_parser_fails_and_second_succeeds() {
        let parser = one_of((fail!(), take(4)));
        let result = parser.parse("test1234");
        assert_eq!(Ok(("1234", "test")), result);
    }

    #[test]
    fn should_work_with_3_parsers() {
        let parser = one_of((fail!(), fail!(), take(4)));
        let result = parser.parse("test1234");
        assert_eq!(Ok(("1234", "test")), result);
    }

    #[test]
    fn should_work_with_4_parsers() {
        let parser = one_of((fail!(), fail!(), fail!(), take(4)));
        let result = parser.parse("test1234");
        assert_eq!(Ok(("1234", "test")), result);
    }
}
