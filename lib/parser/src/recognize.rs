use super::{IsEmpty, Parser, SplitAt, SubsetRange};
use crate::{ConsumedParser, Map, ParseResult, consumed};

// TODO: rename for "parsed"?
pub trait Recognize: Sized {
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
    parser: ConsumedParser<P>,
}

impl<P> RecognizeParser<P> {
    fn new(parser: P) -> Self {
        Self {
            parser: consumed(parser),
        }
    }
}

impl<I, P> Parser<I> for RecognizeParser<P>
where
    I: SubsetRange<I> + SplitAt + Clone + IsEmpty,
    P: Parser<I>,
{
    type Output = I;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output> {
        let parser = |input| self.parser.parse(input);
        parser.map(|(consumed, _)| consumed).parse(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{Map, take, typed_fail, utils::alias};

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
