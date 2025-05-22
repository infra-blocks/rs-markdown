use super::{IsEmpty, Parser, SplitAt, SubsetRange, utils::Reverse};
use crate::parse::parser::ParseResult;

pub trait Recognize: Sized {
    #[allow(dead_code)]
    fn recognize(self) -> RecognizeParser<Self>;
}

impl<T> Recognize for T {
    fn recognize(self) -> RecognizeParser<Self> {
        recognize(self)
    }
}

pub fn recognize<P>(parser: P) -> RecognizeParser<P> {
    RecognizeParser::new(parser)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecognizeParser<P> {
    parser: P,
}

impl<P> RecognizeParser<P> {
    fn new(parser: P) -> Self {
        Self { parser }
    }
}

impl<I, P> Parser<I> for RecognizeParser<P>
where
    I: SubsetRange<I> + SplitAt + Clone + IsEmpty,
    P: Parser<I>,
{
    type Output = I;

    fn parse(&self, input: I) -> ParseResult<I, I> {
        let (remaining, _) = self.parser.parse(input.clone())?;
        if remaining.is_empty() {
            return Ok((remaining, input));
        }
        Ok(input.split_at(input.subset_range(remaining).0).reverse())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parse::{
        parser::{Map, take, typed_fail},
        utils::alias,
    };

    alias!(fail, typed_fail!(&'static str));

    #[test]
    fn should_reject_if_the_underlying_parser_fails() {
        assert_eq!(Err("abc"), recognize(fail!()).parse("abc"));
    }

    #[test]
    fn should_return_the_parsed_value() {
        let parser = take(2).map(|_| 42).recognize();
        assert_eq!(Ok(("c", "ab")), parser.parse("abc"));
    }

    #[test]
    fn should_work_if_the_parser_takes_the_whole_input() {
        let parser = take(3).map(|_| 42).recognize();
        assert_eq!(Ok(("", "abc")), parser.parse("abc"));
    }
}
