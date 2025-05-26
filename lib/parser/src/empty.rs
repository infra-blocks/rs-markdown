use crate::{IsEmpty, ParseResult};

pub fn empty<I>(input: I) -> ParseResult<I, I>
where
    I: IsEmpty + Clone,
{
    if input.is_empty() {
        Ok((input.clone(), input))
    } else {
        Err(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_reject_non_empty_input() {
        assert_eq!(Err(" "), empty(" "));
    }

    #[test]
    fn should_accept_empty_input() {
        assert_eq!(Ok(("", "")), empty(""));
    }
}
