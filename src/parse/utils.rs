use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::take_until,
    character::complete::{line_ending, space0},
    combinator::{rest, verify},
    error::ParseError,
    sequence::terminated,
};

/// Parses the leading indentation of a line, up to 3 spaces.
///
/// Tabs are not allowed, as they count for 4 spaces. See here [CommonMark spec](https://spec.commonmark.org/0.31.2/#tabs).
pub fn indented_by_less_than_4<'a, Error: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, &'a str, Error> {
    verify(space0, |spaces: &str| {
        !spaces.contains("\t") && spaces.len() < 4
    })
    .parse(input)
}

/// Returns a predicate that returns whether the character received is the
/// one provided.
pub fn is_char(c: char) -> impl Fn(char) -> bool {
    move |i| i == c
}

/// Parses a line of text or until the end of the string if found.
///
/// When a terminating line ending sequence is found, it is discarded from the
/// parsed output. This way, whether the line ends with a newline or an abrupt end
/// of input, the parser will produce the same output.
///
/// If the user wishes to determine whether the line ended with a newline or not,
/// they can wrap the parser with [nom::combinator::consumed] or [nom::combinator::recognize].
pub fn line<'a, Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, Error> {
    alt((
        terminated(take_until("\r\n"), line_ending),
        terminated(take_until("\n"), line_ending),
        verify(rest, |s: &str| !s.is_empty()),
    ))
    .parse(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use nom::error::Error;

    mod line {
        use super::*;

        #[test]
        fn should_not_work_with_empty_string() {
            assert!(line::<Error<&str>>("").is_err());
        }

        #[test]
        fn should_work_with_newline() {
            let (remaining, parsed) = line::<Error<&str>>("abc\nstuff").unwrap();
            assert_eq!(remaining, "stuff");
            assert_eq!(parsed, "abc");
        }

        #[test]
        fn should_work_with_bullshit_windows_newline() {
            let (remaining, parsed) = line::<Error<&str>>("abc\r\nstuff").unwrap();
            assert_eq!(remaining, "stuff");
            assert_eq!(parsed, "abc");
        }
    }

    mod indented_by_less_than_4 {
        use super::*;

        #[test]
        fn should_fail_with_tab() {
            assert!(indented_by_less_than_4::<Error<&str>>("\t").is_err());
        }

        #[test]
        fn should_fail_with_4_spaces() {
            assert!(indented_by_less_than_4::<Error<&str>>("    ").is_err());
        }

        #[test]
        fn should_work_with_empty_string() {
            let (remaining, parsed) = indented_by_less_than_4::<Error<&str>>("").unwrap();
            assert_eq!(remaining, "");
            assert_eq!(parsed, "");
        }

        #[test]
        fn should_work_without_indentation() {
            let (remaining, parsed) = indented_by_less_than_4::<Error<&str>>("abc").unwrap();
            assert_eq!(remaining, "abc");
            assert_eq!(parsed, "");
        }

        #[test]
        fn should_work_with_3_spaces() {
            let (remaining, parsed) = indented_by_less_than_4::<Error<&str>>("   abc").unwrap();
            assert_eq!(remaining, "abc");
            assert_eq!(parsed, "   ");
        }
    }
}
