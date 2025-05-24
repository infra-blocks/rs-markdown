use super::{Enumerate, Enumerator, Parser, SplitAt, utils::Reverse};
use crate::ParseResult;
use std::{fmt::Debug, marker::PhantomData};

pub fn take<T>(count: usize) -> TakeParser<T> {
    if count == 0 {
        panic!("chars count must be greater than 0");
    }
    TakeParser::new(count)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TakeParser<T> {
    count: usize,
    _phantom: PhantomData<T>,
}

impl<T> TakeParser<T> {
    fn new(count: usize) -> Self {
        Self {
            count,
            _phantom: PhantomData,
        }
    }

    pub fn that<F>(self, predicate: F) -> TakeThatParser<T, F> {
        TakeThatParser::new(self.count, predicate)
    }
}

impl<I, T> Parser<I> for TakeParser<T>
where
    I: Enumerate<T> + SplitAt,
{
    type Output = I;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output> {
        let mut items_count = 0;
        let mut enumerate = input.enumerate();
        while let Some((_, _)) = enumerate.next() {
            items_count += 1;
            if items_count == self.count {
                return Ok(input.split_at(enumerate.next_index()).reverse());
            }
        }
        // If we make it here, we ran out of input.
        std::mem::drop(enumerate);
        Err(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TakeThatParser<T, F> {
    count: usize,
    predicate: F,
    _phantom: PhantomData<T>,
}

impl<T, F> TakeThatParser<T, F> {
    fn new(count: usize, predicate: F) -> Self {
        Self {
            count,
            predicate,
            _phantom: PhantomData,
        }
    }
}

impl<I, T, F> Parser<I> for TakeThatParser<T, F>
where
    I: Enumerate<T> + SplitAt,
    F: Fn(T) -> bool,
{
    type Output = I;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output> {
        let mut count = 0;
        let mut enumerate = input.enumerate();
        while let Some((index, value)) = enumerate.next() {
            if count == self.count {
                return Ok(input.split_at(index).reverse());
            }
            if !(self.predicate)(value) {
                // If the predicate fails and we have not reached the count yet, we error.
                std::mem::drop(enumerate);
                return Err(input);
            }
            count += 1;
        }
        // If we make it here, we consumed the whole input.
        if count == self.count {
            Ok(input.split_at(input.last_index()).reverse())
        } else {
            std::mem::drop(enumerate);
            Err(input)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn should_panic_for_0() {
        take::<char>(0);
    }

    #[test]
    fn should_reject_when_input_is_smaller() {
        let input = "abc";
        assert_eq!(Err("abc"), take(4).parse(input));
    }

    #[test]
    fn should_work_when_exhausting_input() {
        let input = "abc";
        assert_eq!(Ok(("", "abc")), take(3).parse(input));
    }

    #[test]
    fn should_work_when_subset_of_input() {
        let input = "abc";
        assert_eq!(Ok(("c", "ab")), take(2).parse(input));
    }

    #[test]
    fn should_work_with_unicode() {
        let input = "wörd ist pöpsche";
        assert_eq!(Ok((" ist pöpsche", "wörd")), take(4).parse(input));
    }

    mod take_if {
        use super::*;

        #[test]
        fn should_reject_when_predicate_is_false_before_count() {
            let input = "abc";
            assert_eq!(Err("abc"), take(2).that(|_| false).parse(input));
        }

        #[test]
        fn should_reject_when_input_length_is_smaller_than_count() {
            let input = "abc";
            assert_eq!(Err("abc"), take(4).that(|_| true).parse(input));
        }

        #[test]
        fn should_work_when_exhausting_input() {
            let input = "abc";
            assert_eq!(Ok(("", "abc")), take(3).that(|_| true).parse(input));
        }

        #[test]
        fn should_work_when_subset_of_input() {
            let input = "abc";
            assert_eq!(Ok(("c", "ab")), take(2).that(|_| true).parse(input));
        }
    }
}
