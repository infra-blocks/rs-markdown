use super::{Parser, ParserMut, ParserOnce};
use crate::parse::input::{Input, ParseResult};

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
    use crate::parse::input::ParseQuantity;

    fn test_parser<I: Input<Item = &'static str>>(input: I) -> ParseResult<I, &'static str> {
        let segment = input.segment();
        if segment.len() < 4 {
            return input.failed();
        }
        input.parsed(ParseQuantity::Bytes(4), &segment[..4])
    }

    #[test]
    fn test_should_return_mapped_value_upon_success() {
        let parser = test_parser.map(|s: &str| s.to_uppercase());
        let result = parser.parse("test1234");
        assert_eq!(result, Ok(("1234", "TEST".to_string())));
    }

    #[test]
    fn test_should_not_be_called_upon_failure() {
        let parser = test_parser.map(|_| panic!("you fucked up big time"));
        let result = parser.parse("bad");
        assert_eq!(result, Err("bad"));
    }

    #[test]
    fn should_parse_mut_when_mapping_parser_mut() {
        let mut x = 0;
        let parser = |input: &'static str| {
            x += 1;
            let segment = input;
            input.parsed(ParseQuantity::Bytes(4), &segment[..4])
        };
        let mut parser = parser.map(|s: &str| s.to_uppercase());
        assert_eq!(
            Ok(("1234", "TEST".to_string())),
            parser.parse_mut("test1234")
        );
        assert_eq!(1, x);
    }

    #[test]
    fn should_parse_once_when_mapping_parser_once() {
        let result = vec!["test"];
        let parser = |input: &'static str| input.parsed(ParseQuantity::Bytes(4), result);
        let parser = parser
            .map(|v: Vec<&str>| Iterator::map(v.into_iter(), |s: &str| s.to_uppercase()).collect());
        assert_eq!(
            Ok(("1234", vec!["TEST".to_string()])),
            parser.parse_once("test1234")
        );
    }
}
