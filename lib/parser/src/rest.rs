use super::{ParseResult, SplitAt, utils::Reverse};

pub fn rest<I: SplitAt>(input: I) -> ParseResult<I, I> {
    Ok(input.split_at(input.last_index()).reverse())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_work_for_empty_input() {
        assert_eq!(Ok(("", "")), rest(""));
    }

    #[test]
    fn should_work_for_non_empty_input() {
        assert_eq!(Ok(("", "abc")), rest("abc"));
    }
}
