use crate::ParseResult;
use crate::Parser;

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
    P: Parser<I>,
    F: Fn(P::Output) -> O,
{
    type Output = O;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output> {
        let (remaining, parsed) = self.parser.parse(input)?;
        Ok((remaining, (self.func)(parsed)))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::take;

    #[test]
    fn test_should_return_mapped_value_upon_success() {
        let parser = take(4).map(|s: &str| s.to_uppercase());
        let result = parser.parse("test1234");
        assert_eq!(Ok(("1234", "TEST".to_string())), result);
    }

    #[test]
    fn test_should_not_be_called_upon_failure() {
        let parser = take(4).map(|_| panic!("you fucked up big time"));
        let result = parser.parse("bad");
        assert_eq!(Err("bad"), result);
    }
}
