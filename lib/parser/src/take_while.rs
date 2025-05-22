use super::{Enumerate, ParseResult, Parser, SplitAt, utils::Reverse};

pub fn take_while<E, F>(predicate: F) -> TakeWhileParser<E, F> {
    TakeWhileParser::new(predicate)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TakeWhileParser<E, F> {
    _phantom: std::marker::PhantomData<E>,
    predicate: F,
}

impl<E, F> TakeWhileParser<E, F> {
    fn new(predicate: F) -> Self {
        Self {
            _phantom: std::marker::PhantomData,
            predicate,
        }
    }
}

impl<I, E, F> Parser<I> for TakeWhileParser<E, F>
where
    I: Enumerate<E> + SplitAt,
    F: Fn(E) -> bool,
{
    type Output = I;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output> {
        for (index, item) in input.enumerate() {
            if !(self.predicate)(item) {
                return Ok(input.split_at(index).reverse());
            }
        }
        // If we make it here, we ran out of input and all items succeeded the predicate check.
        Ok(input.split_at(input.last_index()).reverse())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_succeed_when_the_predicate_never_matches() {
        let input = "abc";
        let parser = take_while(|_| false);
        assert_eq!(Ok(("abc", "")), parser.parse(input));
    }

    #[test]
    fn should_succeed_when_the_predicate_matches_all_input() {
        let input = "abc";
        let parser = take_while(|_| true);
        assert_eq!(Ok(("", "abc")), parser.parse(input));
    }

    #[test]
    fn should_succeed_when_the_predicate_matches_subset() {
        let input = "abc";
        let parser = take_while(|c| c == 'a' || c == 'b');
        assert_eq!(Ok(("c", "ab")), parser.parse(input));
    }
}
