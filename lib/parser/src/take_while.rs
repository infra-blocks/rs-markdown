use super::{Enumerate, ParseResult, Parser, SplitAt, utils::Reverse};
use std::mem;

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

    pub fn at_most(self, max: usize) -> TakeWhileAtMostParser<E, F> {
        TakeWhileAtMostParser::new(max, self.predicate)
    }

    pub fn at_least(self, min: usize) -> TakeWhileAtLeastParser<E, F> {
        TakeWhileAtLeastParser::new(min, self.predicate)
    }

    pub fn between(self, min: usize, max: usize) -> TakeWhileBetweenParser<E, F> {
        TakeWhileBetweenParser::new(min, max, self.predicate)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TakeWhileAtMostParser<E, F> {
    max: usize,
    predicate: F,
    _phantom: std::marker::PhantomData<E>,
}

impl<E, F> TakeWhileAtMostParser<E, F> {
    pub fn new(max: usize, predicate: F) -> Self {
        if max == 0 {
            panic!("max must be greater than 0");
        }
        Self {
            max,
            predicate,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn at_least(self, min: usize) -> TakeWhileBetweenParser<E, F> {
        TakeWhileBetweenParser::new(min, self.max, self.predicate)
    }
}

impl<I, E, F> Parser<I> for TakeWhileAtMostParser<E, F>
where
    I: Enumerate<E> + SplitAt,
    F: Fn(E) -> bool,
{
    type Output = I;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output> {
        for (count, (index, item)) in input.enumerate().enumerate() {
            if !(self.predicate)(item) || count == self.max {
                return Ok(input.split_at(index).reverse());
            }
        }
        // If we make it here, we ran out of input and all items succeeded the predicate check.
        Ok(input.split_at(input.last_index()).reverse())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TakeWhileAtLeastParser<E, F> {
    min: usize,
    predicate: F,
    _phantom: std::marker::PhantomData<E>,
}

impl<E, F> TakeWhileAtLeastParser<E, F> {
    pub fn new(min: usize, predicate: F) -> Self {
        if min == 0 {
            panic!("min must be greater than 0");
        }
        Self {
            min,
            predicate,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn at_most(self, max: usize) -> TakeWhileBetweenParser<E, F> {
        TakeWhileBetweenParser::new(self.min, max, self.predicate)
    }
}

impl<I, E, F> Parser<I> for TakeWhileAtLeastParser<E, F>
where
    I: Enumerate<E> + SplitAt,
    F: Fn(E) -> bool,
{
    type Output = I;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output> {
        let mut count = 0;
        let mut enumerate = input.enumerate();
        while let Some((index, item)) = enumerate.next() {
            if !(self.predicate)(item) {
                if count < self.min {
                    mem::drop(enumerate);
                    return Err(input);
                } else {
                    return Ok(input.split_at(index).reverse());
                }
            }
            count += 1;
        }
        if count < self.min {
            mem::drop(enumerate);
            return Err(input);
        }
        // If we make it here, we ran out of input and all items succeeded the predicate check.
        Ok(input.split_at(input.last_index()).reverse())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TakeWhileBetweenParser<E, F> {
    min: usize,
    max: usize,
    predicate: F,
    _phantom: std::marker::PhantomData<E>,
}

impl<E, F> TakeWhileBetweenParser<E, F> {
    pub fn new(min: usize, max: usize, predicate: F) -> Self {
        if min == 0 {
            panic!("min must be greater than 0");
        }
        if max == 0 {
            panic!("max must be greater than 0");
        }
        if min > max {
            panic!("min must be less than or equal to max");
        }

        Self {
            min,
            max,
            predicate,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<I, E, F> Parser<I> for TakeWhileBetweenParser<E, F>
where
    I: Enumerate<E> + SplitAt,
    F: Fn(E) -> bool,
{
    type Output = I;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output> {
        let mut count = 0;
        let mut enumerate = input.enumerate();
        while let Some((index, item)) = enumerate.next() {
            if !(self.predicate)(item) || count == self.max {
                if count < self.min {
                    mem::drop(enumerate);
                    return Err(input);
                } else {
                    return Ok(input.split_at(index).reverse());
                }
            }
            count += 1;
        }
        if count < self.min {
            mem::drop(enumerate);
            return Err(input);
        }
        // If we make it here, we ran out of input and all items succeeded the predicate check.
        Ok(input.split_at(input.last_index()).reverse())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sould_succeed_when_the_input_is_empty() {
        let input = "";
        let parser = take_while(|_| true);
        assert_eq!(Ok(("", "")), parser.parse(input));
    }

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

    mod at_most {
        use super::*;
        use std::panic;

        #[test]
        fn should_panic_when_max_is_zero() {
            assert!(panic::catch_unwind(|| take_while(|_| true).at_most(0).parse("toto")).is_err());
        }

        #[test]
        fn should_succeed_when_the_predicate_never_matches() {
            let input = "abc";
            let parser = take_while(|_| false).at_most(1);
            assert_eq!(Ok(("abc", "")), parser.parse(input));
        }

        #[test]
        fn should_stop_when_max_bound_is_smaller_than_input_length() {
            let input = "abc";
            let parser = take_while(|_| true).at_most(1);
            assert_eq!(Ok(("bc", "a")), parser.parse(input));
        }

        #[test]
        fn should_work_when_max_bound_is_equals_to_input_length() {
            let input = "abc";
            let parser = take_while(|_| true).at_most(3);
            assert_eq!(Ok(("", "abc")), parser.parse(input));
        }

        #[test]
        fn should_work_when_max_bound_is_greater_than_input_length() {
            let input = "abc";
            let parser = take_while(|_| true).at_most(4);
            assert_eq!(Ok(("", "abc")), parser.parse(input));
        }
    }

    mod at_least {
        use super::*;
        use std::panic;

        #[test]
        fn should_panic_when_min_is_zero() {
            assert!(
                panic::catch_unwind(|| take_while(|_| true).at_least(0).parse("toto")).is_err()
            );
        }

        #[test]
        fn should_error_if_predicate_is_false_before_min() {
            let input = "a";
            let parser = take_while(|_| false).at_least(1);
            assert_eq!(Err(input), parser.parse(input));
        }

        #[test]
        fn should_error_if_input_length_smaller_than_min() {
            let input = "a";
            let parser = take_while(|_| true).at_least(2);
            assert_eq!(Err(input), parser.parse(input));
        }

        #[test]
        fn should_work_if_min_is_equals_to_input_length() {
            let input = "abc";
            let parser = take_while(|_| true).at_least(3);
            assert_eq!(Ok(("", "abc")), parser.parse(input));
        }

        #[test]
        fn should_work_if_min_is_lower_than_input_length() {
            let input = "abc";
            let parser = take_while(|_| true).at_least(3);
            assert_eq!(Ok(("", "abc")), parser.parse(input));
        }
    }

    mod between {
        use super::*;
        use std::panic;

        #[test]
        fn should_panic_when_min_is_zero() {
            assert!(
                panic::catch_unwind(|| take_while(|_| true).between(0, 1).parse("toto")).is_err()
            );
        }

        #[test]
        fn should_panic_when_max_is_zero() {
            assert!(
                panic::catch_unwind(|| take_while(|_| true).at_least(1).at_most(0).parse("toto"))
                    .is_err()
            );
        }

        #[test]
        fn should_panic_when_min_is_greater_than_max() {
            assert!(
                panic::catch_unwind(|| take_while(|_| true).at_most(1).at_least(2).parse("toto"))
                    .is_err()
            );
        }

        #[test]
        fn should_error_if_input_length_smaller_than_min() {
            let input = "";
            let parser = take_while(|_| true).between(1, 2);
            assert_eq!(Err(input), parser.parse(input));
        }

        #[test]
        fn should_succeed_if_input_length_equals_min_equals_max() {
            let input = "abc";
            let parser = take_while(|_| true).at_least(3).at_most(3);
            assert_eq!(Ok(("", "abc")), parser.parse(input));
        }

        #[test]
        fn should_stop_if_max_is_smaller_than_input_length() {
            let input = "abc";
            let parser = take_while(|_| true).at_least(1).at_most(2);
            assert_eq!(Ok(("c", "ab")), parser.parse(input));
        }

        #[test]
        fn should_work_if_max_is_greater_than_input_length() {
            let input = "abc";
            let parser = take_while(|_| true).at_least(1).at_most(4);
            assert_eq!(Ok(("", "abc")), parser.parse(input));
        }
    }
}
