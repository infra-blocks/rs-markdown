use super::Parser;
use crate::ParseResult;

pub trait Maybe: Sized {
    fn maybe(self) -> MaybeParser<Self>;
}

impl<P> Maybe for P {
    fn maybe(self) -> MaybeParser<Self> {
        maybe(self)
    }
}

pub fn maybe<P>(parser: P) -> MaybeParser<P> {
    MaybeParser::new(parser)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaybeParser<P> {
    parser: P,
}

impl<P> MaybeParser<P> {
    fn new(parser: P) -> Self {
        Self { parser }
    }
}

impl<I, P> Parser<I> for MaybeParser<P>
where
    P: Parser<I>,
{
    type Output = Option<P::Output>;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output> {
        match self.parser.parse(input) {
            Ok((remaining, output)) => Ok((remaining, Some(output))),
            // Turns an error into a none.
            Err(input) => Ok((input, None)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{Map, take, typed_fail, utils::alias};

    alias!(fail, typed_fail!(&'static str));

    #[test]
    fn should_return_none_if_the_inner_parser_fails() {
        assert_eq!(Ok(("abc", None)), maybe(fail!()).parse("abc"));
    }

    #[test]
    fn should_return_some_when_inner_parser_succeeds() {
        let parser = take(2).map(|_| 42).maybe();
        assert_eq!(Ok(("c", Some(42))), parser.parse("abc"));
    }
}
