use super::Parser;
use crate::parse::parser::ParseResult;

pub trait Validate<F>: Sized {
    fn validate(self, func: F) -> ValidateParser<Self, F>;
}

impl<F, T> Validate<F> for T {
    fn validate(self, func: F) -> ValidateParser<Self, F> {
        validate(self, func)
    }
}

pub fn validate<P, F>(parser: P, func: F) -> ValidateParser<P, F> {
    ValidateParser::new(parser, func)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidateParser<P, F> {
    parser: P,
    func: F,
}

impl<P, F> ValidateParser<P, F> {
    fn new(parser: P, func: F) -> Self {
        Self { parser, func }
    }
}

impl<I, P, F> Parser<I> for ValidateParser<P, F>
where
    I: Clone,
    P: Parser<I>,
    F: Fn(&P::Output) -> bool,
{
    type Output = P::Output;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output> {
        let (remaining, parsed) = self.parser.parse(input.clone())?;
        if (self.func)(&parsed) {
            Ok((remaining, parsed))
        } else {
            Err(input)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parse::parser::fail::typed_fail;
    use crate::parse::parser::take;
    use crate::parse::utils::alias;

    alias!(fail, typed_fail![&'static str]);

    #[test]
    fn test_rejects_when_parser_rejects_and_does_not_call_predicate() {
        let parser = fail!().validate(|_: &&str| {
            panic!("fucked up big time here");
        });
        assert_eq!(Err("test1234"), parser.parse("test1234"));
    }

    #[test]
    fn test_rejects_when_parser_succeeds_but_predicate_returns_false() {
        let parser = take(4).validate(|_: &&str| false);
        assert_eq!(Err("test1234"), parser.parse("test1234"));
    }

    #[test]
    fn test_succeeds_when_parser_succeeds_and_predicate_returns_true() {
        let parser = take(4).validate(|_: &&str| true);
        assert_eq!(Ok(("1234", "test")), parser.parse("test1234"));
    }
}
