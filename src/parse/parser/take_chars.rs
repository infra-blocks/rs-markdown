use super::{Parser, ParserMut, ParserOnce};
use crate::parse::{
    ParseResult,
    input::{Input, ParseQuantity},
};
use nom::AsChar;

pub fn take_chars(count: usize) -> TakeChars {
    if count == 0 {
        panic!("chars count must be greater than 0");
    }
    TakeChars::new(count)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TakeChars {
    count: usize,
}

impl TakeChars {
    fn new(count: usize) -> Self {
        Self { count }
    }
}

impl<'a, I> Parser<I> for TakeChars
where
    I: Input<Item = &'a str>,
{
    type Output = &'a str;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output> {
        let Some(item) = input.first() else {
            return input.failed();
        };
        if item.chars().count() < self.count {
            return input.failed();
        }
        let offset = item.chars().take(self.count).map(|char| char.len()).sum();
        input.parsed(ParseQuantity::Bytes(offset), &item[..offset])
    }
}

impl<'a, I> ParserMut<I> for TakeChars
where
    I: Input<Item = &'a str>,
{
    type Output = &'a str;

    fn parse_mut(&mut self, input: I) -> ParseResult<I, Self::Output> {
        self.parse(input)
    }
}

impl<'a, I> ParserOnce<I> for TakeChars
where
    I: Input<Item = &'a str>,
{
    type Output = &'a str;

    fn parse_once(self, input: I) -> ParseResult<I, Self::Output> {
        self.parse(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn should_panic_for_0() {
        take_chars(0);
    }

    #[test]
    fn should_reject_when_input_is_smaller() {
        let input = "abc";
        assert_eq!(Err("abc"), take_chars(4).parse(input));
    }

    #[test]
    fn should_work_when_exhausting_input() {
        let input = "abc";
        assert_eq!(Ok(("", "abc")), take_chars(3).parse(input));
    }

    #[test]
    fn should_work_when_subset_of_input() {
        let input = "abc";
        assert_eq!(Ok(("c", "ab")), take_chars(2).parse(input));
    }

    #[test]
    fn should_work_with_unicode() {
        let input = "wörd ist pöpsche";
        assert_eq!(Ok((" ist pöpsche", "wörd")), take_chars(4).parse(input));
    }
}
