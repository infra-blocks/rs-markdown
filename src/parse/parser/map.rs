use super::{Parser, ParserMut, ParserOnce};
use crate::parse::{ParseResult, input::Input};

pub trait Map<F>: Sized {
    fn map(self, func: F) -> MapParser<Self, F>;
}

impl<F, T> Map<F> for T {
    fn map(self, func: F) -> MapParser<Self, F> {
        map(self, func)
    }
}

pub fn map<P, F>(parser: P, func: F) -> MapParser<P, F> {
    MapParser::new(parser, func)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MapParser<P, F> {
    parser: P,
    func: F,
}

impl<P, F> MapParser<P, F> {
    fn new(parser: P, func: F) -> Self {
        Self { parser, func }
    }
}

impl<I, O, P, F> Parser<I> for MapParser<P, F>
where
    I: Input,
    P: Parser<I>,
    F: Fn(P::Output) -> O,
{
    type Output = O;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output> {
        let (remaining, parsed) = self.parser.parse(input)?;
        Ok((remaining, (self.func)(parsed)))
    }
}

impl<I, O, P, F> ParserMut<I> for MapParser<P, F>
where
    I: Input,
    P: ParserMut<I>,
    F: FnMut(P::Output) -> O,
{
    type Output = O;

    fn parse_mut(&mut self, input: I) -> ParseResult<I, Self::Output> {
        let (remaining, parsed) = self.parser.parse_mut(input)?;
        Ok((remaining, (self.func)(parsed)))
    }
}

impl<I, O, P, F> ParserOnce<I> for MapParser<P, F>
where
    I: Input,
    P: ParserOnce<I>,
    F: FnOnce(P::Output) -> O,
{
    type Output = O;

    fn parse_once(self, input: I) -> ParseResult<I, Self::Output> {
        let (remaining, parsed) = self.parser.parse_once(input)?;
        Ok((remaining, (self.func)(parsed)))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parse::{lines, parser::take_chars};

    #[test]
    fn test_should_return_mapped_value_upon_success() {
        let parser = take_chars(4).map(|s: &str| s.to_uppercase());
        let result = parser.parse(lines!("test1234"));
        assert_eq!(Ok((lines!("1234"), "TEST".to_string())), result);
    }

    #[test]
    fn test_should_not_be_called_upon_failure() {
        let parser = take_chars(4).map(|_| panic!("you fucked up big time"));
        let result = parser.parse(lines!("bad"));
        assert_eq!(Err(lines!("bad")), result);
    }

    #[test]
    fn should_parse_mut_when_mapping_parser_mut() {
        let mut parser = take_chars(4);
        let parser = |input| parser.parse_mut(input);
        let mut parser = parser.map(|s: &str| s.to_uppercase());
        let result = parser.parse_mut(lines!("test1234"));
        assert_eq!(Ok((lines!("1234"), "TEST".to_string())), result);
    }

    #[test]
    fn should_parse_once_when_mapping_parser_once() {
        let parser = take_chars(4);
        let parser = |input| parser.parse_once(input);
        let parser = parser.map(|s: &str| s.to_ascii_uppercase());
        let result = parser.parse_once(lines!("test1234"));
        assert_eq!(Ok((lines!("1234"), "TEST".to_string())), result);
    }
}
