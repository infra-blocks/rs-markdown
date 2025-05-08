use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{is_not, take_until, take_while_m_n},
    character::{
        anychar,
        complete::{char, line_ending, space0, space1},
    },
    combinator::{recognize, rest, verify},
    error::ParseError,
    sequence::terminated,
};

/// Parses any escaped character sequence.
///
/// An escaped character sequence is a backslash character followed by any other character.
/// This parser always matches 2 characters, or fails.
pub fn escaped_sequence<'a, Error: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, &'a str, Error> {
    recognize((char('\\'), anychar)).parse(input)
}

/// Parses and consumes all spaces and tabs at the beginning of the input,
/// then verifies that the amount of whitespace is at least 4.
///
/// This space scoring technique is described in the [CommonMark spec](https://spec.commonmark.org/0.31.2/#tabs).
pub fn indented_by_at_least_4<'a, Error: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, &'a str, Error> {
    verify(space1, |spaces: &str| {
        spaces.contains("\t") || spaces.len() >= 4
    })
    .parse(input)
}

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

/// Returns a predicate that is true when the character matches one of the characters
/// provided as input.
pub fn is_one_of(chars: &[char]) -> impl Fn(char) -> bool + use<'_> {
    move |i| chars.contains(&i)
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

/// Parses everything until the first whitespace character.
///
/// Here, the whitespace characters are defined as: space, tab, carriage return and newline.
pub fn non_whitespace<'a, Error: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, &'a str, Error> {
    is_not(" \t\r\n").parse(input)
}

/// Takes a single character matching the predicate provided.
///
/// It's very similar to the [nom::character::one_of] parser, but uses a predicate
/// instead of a list of characters.
pub fn take_one<'a, F, Error>(predicate: F) -> impl Parser<&'a str, Output = char, Error = Error>
where
    F: Fn(char) -> bool,
    Error: ParseError<&'a str>,
{
    take_while_m_n(1, 1, predicate).map(|s: &str| {
        assert!(s.len() == 1);
        s.chars().next().unwrap()
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use nom::error::Error;

    mod escaped_sequence {
        use super::*;

        #[test]
        fn should_fail_with_empty_string() {
            assert!(escaped_sequence::<Error<&str>>("").is_err());
        }

        #[test]
        fn should_fail_with_just_backslash() {
            assert!(escaped_sequence::<Error<&str>>("\\").is_err());
        }

        #[test]
        fn should_fail_with_any_other_single_character() {
            assert!(escaped_sequence::<Error<&str>>("a").is_err());
        }

        #[test]
        fn should_fail_with_an_unescaped_pair_of_characters() {
            assert!(escaped_sequence::<Error<&str>>("a\\").is_err());
        }

        #[test]
        fn should_work_with_double_backslash() {
            let (remaining, parsed) = escaped_sequence::<Error<&str>>("\\\\").unwrap();
            assert_eq!(remaining, "");
            assert_eq!(parsed, "\\\\");
        }

        #[test]
        fn should_work_with_escaped_ascii_character() {
            let (remaining, parsed) = escaped_sequence::<Error<&str>>("\\a").unwrap();
            assert_eq!(remaining, "");
            assert_eq!(parsed, "\\a");
        }

        #[test]
        fn should_work_with_escaped_unicode_character() {
            let (remaining, parsed) = escaped_sequence::<Error<&str>>("\\é").unwrap();
            assert_eq!(remaining, "");
            assert_eq!(parsed, "\\é");
        }
    }

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

    mod indented_by_at_least_4 {
        use super::*;

        #[test]
        fn should_fail_with_empty_string() {
            assert!(indented_by_at_least_4::<Error<&str>>("").is_err());
        }

        #[test]
        fn should_fail_with_3_spaces() {
            assert!(indented_by_at_least_4::<Error<&str>>("   ").is_err());
        }

        #[test]
        fn should_fail_with_non_whitespace_character() {
            assert!(indented_by_at_least_4::<Error<&str>>("   a   ").is_err());
        }

        #[test]
        fn should_work_with_a_tab() {
            let (remaining, parsed) = indented_by_at_least_4::<Error<&str>>("\tabc").unwrap();
            assert_eq!(remaining, "abc");
            assert_eq!(parsed, "\t");
        }

        #[test]
        fn should_work_with_4_spaces() {
            let (remaining, parsed) = indented_by_at_least_4::<Error<&str>>("    abc").unwrap();
            assert_eq!(remaining, "abc");
            assert_eq!(parsed, "    ");
        }

        #[test]
        fn should_work_with_a_mix() {
            let (remaining, parsed) =
                indented_by_at_least_4::<Error<&str>>("   \t  \t  toto").unwrap();
            assert_eq!(remaining, "toto");
            assert_eq!(parsed, "   \t  \t  ");
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

    mod non_whitespace {
        use super::*;

        #[test]
        fn should_fail_with_empty_string() {
            assert!(non_whitespace::<Error<&str>>("").is_err());
        }

        #[test]
        fn should_fail_with_space() {
            assert!(non_whitespace::<Error<&str>>(" ").is_err());
        }

        #[test]
        fn should_fail_with_tab() {
            assert!(non_whitespace::<Error<&str>>("\t").is_err());
        }

        #[test]
        fn should_fail_with_carriage_return() {
            assert!(non_whitespace::<Error<&str>>("\r").is_err());
        }

        #[test]
        fn should_fail_with_newline() {
            assert!(non_whitespace::<Error<&str>>("\n").is_err());
        }

        #[test]
        fn should_consume_everything_until_whitespace() {
            let (remaining, parsed) = non_whitespace::<Error<&str>>("abc def").unwrap();
            assert_eq!(remaining, " def");
            assert_eq!(parsed, "abc");
        }
    }

    mod take_one {
        use super::*;

        #[test]
        fn should_fail_with_empty_string() {
            assert!(take_one::<_, Error<&str>>(is_char('a')).parse("").is_err());
        }

        #[test]
        fn should_fail_with_non_matching_character() {
            assert!(take_one::<_, Error<&str>>(is_char('a')).parse("b").is_err());
        }

        #[test]
        fn should_work_with_matching_character() {
            let (remaining, parsed) = take_one::<_, Error<&str>>(|c| c.is_ascii_control())
                .parse("\x00ab")
                .unwrap();
            assert_eq!(remaining, "ab");
            assert_eq!(parsed, '\x00');
        }
    }
}
