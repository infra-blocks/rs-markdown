use super::{Parser, PrefixEnd, SplitAt, utils::Reverse};
use crate::parse::parser::ParseResult;

pub fn tag<T>(tag: T) -> TagParser<T> {
    TagParser::new(tag)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TagParser<T> {
    tag: T,
}

impl<T> TagParser<T> {
    fn new(tag: T) -> Self {
        Self { tag }
    }
}

impl<I, T> Parser<I> for TagParser<T>
where
    I: SplitAt + PrefixEnd<T>,
    T: Clone,
{
    type Output = I;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output> {
        match input.prefix_end(self.tag.clone()) {
            Some(index) => Ok(input.split_at(index).reverse()),
            None => Err(input),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_tag_should_match_anything() {
        assert_eq!(Ok(("abc", "")), tag("").parse("abc"));
    }

    #[test]
    fn should_work_for_subset_prefix() {
        assert_eq!(Ok(("c", "ab")), tag("ab").parse("abc"));
    }

    #[test]
    fn should_work_for_whole_input() {
        assert_eq!(Ok(("", "abc")), tag("abc").parse("abc"));
    }

    #[test]
    fn should_fail_when_longer() {
        assert_eq!(Err("abc"), tag("abcd").parse("abc"));
    }

    #[test]
    fn should_fail_when_not_prefix() {
        assert_eq!(Err("abc"), tag("bc").parse("abc"));
    }
}
