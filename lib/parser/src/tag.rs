use super::{Parser, SplitAt, utils::Reverse};
use crate::{ItemsIndices, ParseResult};

pub fn tag<T, U>(tag: T) -> TagParser<T, U> {
    TagParser::new(tag)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TagParser<T, U> {
    tag: T,
    _phantom: std::marker::PhantomData<U>,
}

impl<T, U> TagParser<T, U> {
    fn new(tag: T) -> Self {
        Self {
            tag,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<I, T, U> Parser<I> for TagParser<T, U>
where
    U: PartialEq,
    I: SplitAt + ItemsIndices<U>,
    T: ItemsIndices<U>,
{
    type Output = I;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output> {
        match input.after_prefix(self.tag.items()) {
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
