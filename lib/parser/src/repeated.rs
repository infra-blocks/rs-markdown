use crate::ParseResult;
use crate::Parser;

pub trait Repeated: Sized {
    fn repeated(self) -> RepeatedParser<Self>;
}

impl<T> Repeated for T {
    fn repeated(self) -> RepeatedParser<Self> {
        repeated(self)
    }
}

pub fn repeated<T>(parser: T) -> RepeatedParser<T> {
    RepeatedParser::new(parser)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RepeatedParser<P> {
    parser: P,
}

impl<P> RepeatedParser<P> {
    fn new(parser: P) -> Self {
        Self { parser }
    }
}

impl<P> RepeatedParser<P> {
    pub fn at_least(self, min: usize) -> RepeatedAtLeastParser<P> {
        RepeatedAtLeastParser::new(min, self.parser)
    }
}

impl<I, P> Parser<I> for RepeatedParser<P>
where
    P: Parser<I>,
{
    type Output = Vec<P::Output>;

    fn parse(&self, input: I) -> ParseResult<I, Vec<P::Output>> {
        let mut remaining = input;
        let mut results = Vec::new();

        let remaining = loop {
            match self.parser.parse(remaining) {
                Ok((next_remaining, parsed)) => {
                    results.push(parsed);
                    remaining = next_remaining;
                }
                Err(remaining) => break remaining,
            }
        };

        Ok((remaining, results))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RepeatedAtLeastParser<P> {
    min: usize,
    parser: P,
}

impl<P> RepeatedAtLeastParser<P> {
    fn new(min: usize, parser: P) -> Self {
        if min == 0 {
            panic!("min must be greater than 0");
        }

        Self { min, parser }
    }
}

impl<I, P> Parser<I> for RepeatedAtLeastParser<P>
where
    I: Clone,
    P: Parser<I>,
{
    type Output = Vec<P::Output>;

    fn parse(&self, input: I) -> ParseResult<I, Vec<P::Output>> {
        let mut remaining = input.clone();
        let mut results = Vec::new();

        let mut count = 0;
        let remaining = loop {
            match self.parser.parse(remaining) {
                Ok((next_remaining, parsed)) => {
                    results.push(parsed);
                    remaining = next_remaining;
                    count += 1;
                }
                Err(remaining) => break remaining,
            }
        };
        if count < self.min {
            return Err(input);
        }

        Ok((remaining, results))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{take, typed_fail, utils::alias};

    alias!(fail, typed_fail!(&'static str));

    #[test]
    fn test_should_return_empty_array_on_failure() {
        let parser = fail!().repeated();
        let result = parser.parse("test1234");
        assert_eq!(Ok(("test1234", vec![])), result);
    }

    #[test]
    fn test_should_succeed_if_it_can_parse_one() {
        let parser = take(4).repeated();
        let result = parser.parse("test12");
        assert_eq!(Ok(("12", vec!["test"])), result);
    }

    #[test]
    fn test_should_return_as_many_values_as_possible() {
        let parser = take(4).repeated();
        let result = parser.parse("test123456");
        assert_eq!(Ok(("56", vec!["test", "1234"])), result);
    }

    mod at_least {
        use super::*;

        #[test]
        fn should_error_parsed_count_lower_than_min() {
            let parser = repeated(take(4)).at_least(3);
            let result = parser.parse("test1234");
            assert_eq!(Err("test1234"), result);
        }

        #[test]
        fn should_work_if_min_smaller_than_parsed_count() {
            let parser = take(4).repeated().at_least(1);
            let result = parser.parse("test1234");
            assert_eq!(Ok(("", vec!["test", "1234"])), result);
        }

        #[test]
        fn should_work_if_min_equals_parsed_count() {
            let parser = take(4).repeated().at_least(2);
            let result = parser.parse("test1234");
            assert_eq!(Ok(("", vec!["test", "1234"])), result);
        }
    }
}
