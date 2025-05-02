use nom::{
    branch::alt,
    bytes::complete::{take_until, take_while_m_n},
    character::complete::line_ending,
    combinator::rest,
    error::ParseError,
    sequence::terminated,
    Parser,
};

/// Returns a predicate that returns whether the character received is the
/// one provided.
pub fn is_char(c: char) -> impl Fn(char) -> bool {
    move |i| i == c
}

/// Returns a predicate that is true when the character matches one of the characters
/// provided as input.
pub fn is_one_of(chars: &[char]) -> impl Fn(char) -> bool {
    let chars = chars.to_vec();
    move |i| chars.contains(&i)
}

/// Parses a line of text or until the end of the string if found.
///
/// When a terminating line ending sequence is found, it is discarded from the
/// parsed output. This way, whether the line ends with a newline or an abrupt end
/// of input, the parser will produce the same output.
///
/// If the user wishes to determine whether the line ended with a newline or not,
/// they can wrap the parser with [nom::combinator::consumed].
pub fn line<'a, Error: ParseError<&'a str>>(
) -> impl Parser<&'a str, Output = &'a str, Error = Error> {
    alt((
        terminated(take_until("\r\n"), line_ending),
        terminated(take_until("\n"), line_ending),
        rest,
    ))
}

/// Parses the first n whitespace characters.
///
/// The only whitespace character used at this time is the space character.
pub fn up_to_n_whitespace<'a, Error: ParseError<&'a str>>(
    count: usize,
) -> impl Parser<&'a str, Output = &'a str, Error = Error> {
    take_while_m_n(0, count, |c| c == ' ')
}

#[cfg(test)]
mod test {
    use super::*;

    mod line {
        use nom::error::Error;

        use super::*;

        #[test]
        fn should_work_with_empty_string() {
            let (remaining, parsed) = line::<Error<&str>>().parse("").unwrap();
            assert_eq!(remaining, "");
            assert_eq!(parsed, "");
        }

        #[test]
        fn should_work_with_newline() {
            let (remaining, parsed) = line::<Error<&str>>().parse("abc\nstuff").unwrap();
            assert_eq!(remaining, "stuff");
            assert_eq!(parsed, "abc");
        }

        #[test]
        fn should_work_with_bullshit_windows_newline() {
            let (remaining, parsed) = line::<Error<&str>>().parse("abc\r\nstuff").unwrap();
            assert_eq!(remaining, "stuff");
            assert_eq!(parsed, "abc");
        }
    }

    mod up_to_n_whitespace {
        use nom::error::Error;

        use super::*;

        #[test]
        fn should_work_with_empty_string() {
            let (remaining, parsed) = up_to_n_whitespace::<Error<&str>>(1).parse("").unwrap();
            assert_eq!(remaining, "");
            assert_eq!(parsed, "");
        }

        #[test]
        fn should_work_without_space() {
            let (remaining, parsed) = up_to_n_whitespace::<Error<&str>>(3).parse("abc").unwrap();
            assert_eq!(remaining, "abc");
            assert_eq!(parsed, "");
        }

        #[test]
        fn should_work_with_less_spaces() {
            let (remaining, parsed) = up_to_n_whitespace::<Error<&str>>(3).parse("  abc").unwrap();
            assert_eq!(remaining, "abc");
            assert_eq!(parsed, "  ");
        }

        #[test]
        fn should_work_with_exact_spaces() {
            let (remaining, parsed) = up_to_n_whitespace::<Error<&str>>(3)
                .parse("   abc")
                .unwrap();
            assert_eq!(remaining, "abc");
            assert_eq!(parsed, "   ");
        }

        #[test]
        fn should_work_with_more_spaces() {
            let (remaining, parsed) = up_to_n_whitespace::<Error<&str>>(3)
                .parse("     abc")
                .unwrap();
            assert_eq!(remaining, "  abc");
            assert_eq!(parsed, "   ");
        }
    }
}
