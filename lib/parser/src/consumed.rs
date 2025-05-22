use super::{IsEmpty, Parser, SplitAt, SubsetRange};
use crate::ParseResult;

pub trait Consumed: Sized {
    fn consumed(self) -> ConsumedParser<Self>;
}

impl<T> Consumed for T {
    fn consumed(self) -> ConsumedParser<Self> {
        consumed(self)
    }
}

pub fn consumed<P>(parser: P) -> ConsumedParser<P> {
    ConsumedParser::new(parser)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConsumedParser<P> {
    parser: P,
}

impl<P> ConsumedParser<P> {
    fn new(parser: P) -> Self {
        Self { parser }
    }
}

impl<I, P> Parser<I> for ConsumedParser<P>
where
    I: SubsetRange<I> + SplitAt + Clone + IsEmpty,
    P: Parser<I>,
{
    type Output = (I, P::Output);

    fn parse(&self, input: I) -> ParseResult<I, Self::Output> {
        let (remaining, parsed) = self.parser.parse(input.clone())?;
        // If we have consumed the whole input.
        if remaining.is_empty() {
            return Ok((remaining, (input, parsed)));
        }
        let (consumed, _) = input.split_at(input.subset_range(remaining.clone()).0);
        Ok((remaining, (consumed, parsed)))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{Map, take, typed_fail, utils::alias};

    alias!(fail, typed_fail!(&'static str));

    #[test]
    fn should_reject_if_the_underlying_parser_fails() {
        assert_eq!(Err("abc"), consumed(fail!()).parse("abc"));
    }

    #[test]
    fn should_work_if_the_parser_takes_a_subset_of_the_input() {
        let parser = take(2).map(|_| 42).consumed();
        assert_eq!(Ok(("c", ("ab", 42))), parser.parse("abc"));
    }

    #[test]
    fn should_work_if_the_parser_takes_the_whole_input() {
        let parser = take(3).map(|_| 42).consumed();
        assert_eq!(Ok(("", ("abc", 42))), parser.parse("abc"));
    }
}
