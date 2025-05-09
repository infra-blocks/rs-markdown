use crate::parse::{
    traits::{Parse, Segment},
    utils::{parentheseses_balance, take_one},
};
use nom::{
    IResult, Parser,
    bytes::complete::take_while,
    combinator::{recognize, verify},
    error::ParseError,
};

/// A unbracketed link destination segment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnbracketedLinkDestinationSegment<'a>(&'a str);

impl<'a> UnbracketedLinkDestinationSegment<'a> {
    fn new(segment: &'a str) -> Self {
        Self(segment)
    }
}

/*
From the spec, a "unbracketed" link destination is:
a nonempty sequence of characters that does not start with <, does not include ASCII control characters or space character,
and includes parentheses only if (a) they are backslash-escaped or (b) they are part of a balanced pair of unescaped parentheses.
(Implementations may impose limits on parentheses nesting to avoid performance issues, but at least three levels of nesting should be supported.)
*/
impl<'a> Parse<'a> for UnbracketedLinkDestinationSegment<'a> {
    fn parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error>
    where
        Self: Sized,
    {
        verify(
            recognize((
                take_one(utils::is_opening_char),
                take_while(utils::is_continuation_char),
            )),
            parentheseses_balance,
        )
        .map(Self::new)
        .parse(input)
    }
}

impl<'a> Segment<'a> for UnbracketedLinkDestinationSegment<'a> {
    fn segment(&self) -> &'a str {
        self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod parse {
        use super::*;
        use nom::error::Error;

        macro_rules! failure_case {
            ($test:ident, $segment:expr) => {
                #[test]
                fn $test() {
                    assert!(
                        UnbracketedLinkDestinationSegment::parse::<Error<&str>>($segment).is_err()
                    );
                }
            };
        }

        macro_rules! success_case {
            ($test:ident, $segment:expr) => {
                success_case!($test, $segment, $segment, "");
            };
            ($test:ident, $segment:expr, $parsed:expr, $remaining:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(
                        UnbracketedLinkDestinationSegment::parse::<Error<&str>>($segment),
                        Ok(($remaining, UnbracketedLinkDestinationSegment::new($parsed)))
                    );
                }
            };
        }

        failure_case!(should_reject_empty_string, "");
        failure_case!(should_reject_blank_line, "\n");
        failure_case!(should_reject_space, " ");
        failure_case!(should_reject_leading_whitespace, " a");
        failure_case!(should_reject_ascii_control_character, "\x00");
        failure_case!(should_reject_unbalanced_parentheses, "(foo(and(bar))");

        success_case!(should_work_with_character, "a");
        success_case!(should_work_with_several_characters, "abc");
        success_case!(should_work_with_slash, "/");
        success_case!(should_work_with_relative_path, "./relative/path.sftu");
        success_case!(should_work_with_fragment_identifier, "#fragment");
        success_case!(
            should_work_with_full_uri,
            "https://example.com?query=value#head-wallet"
        );
        success_case!(
            should_work_with_balanced_parentheses,
            r"(foo\(blip(and(bar)))"
        );
        success_case!(should_stop_at_first_space, "foo bar", "foo", " bar");
        success_case!(should_stop_at_first_newline, "foo\nbar", "foo", "\nbar");
    }
}

mod utils {
    pub fn is_opening_char(character: char) -> bool {
        // The segment cannot start with the '<' character.s
        is_continuation_char(character) && character != '<'
    }

    pub fn is_continuation_char(character: char) -> bool {
        // ASCII control characters and spaces are not allowed
        character != ' ' && !character.is_ascii_control() && character != '\n'
    }

    #[cfg(test)]
    mod test {
        use super::*;

        mod is_opening_char {
            use super::*;

            #[test]
            fn should_reject_opening_bracket() {
                assert!(!is_opening_char('<'));
            }

            #[test]
            fn should_reject_space() {
                assert!(!is_opening_char(' '));
            }

            #[test]
            fn should_reject_newline() {
                assert!(!is_opening_char('\n'));
            }

            #[test]
            fn should_reject_ascii_control_character() {
                assert!(!is_opening_char('\x00'));
            }

            #[test]
            fn should_accept_any_other_character() {
                assert!(is_opening_char('a'));
            }
        }

        mod is_continuation_char {
            use super::*;

            #[test]
            fn should_reject_space() {
                assert!(!is_continuation_char(' '));
            }

            #[test]
            fn should_reject_newline() {
                assert!(!is_continuation_char('\n'));
            }

            #[test]
            fn should_reject_ascii_control_character() {
                assert!(!is_continuation_char('\x00'));
            }

            #[test]
            fn should_accept_any_other_character() {
                assert!(is_continuation_char('a'));
            }

            #[test]
            fn should_accept_opening_angle_bracket() {
                assert!(is_continuation_char('<'));
            }
        }
    }
}
