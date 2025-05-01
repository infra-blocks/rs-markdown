use nom::{
    branch::alt,
    character::complete::{line_ending, space0, space1},
    combinator::{consumed, eof},
    error::ParseError,
    Parser,
};

/// Represents a blank line segment.
///
/// A blank line contains at least one whitespace character, and only whitespace characters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlankLineSegment<'a>(pub &'a str);

impl<'a> BlankLineSegment<'a> {
    fn new(segment: &'a str) -> Self {
        Self(segment)
    }

    pub fn parser<Error: ParseError<&'a str>>() -> impl Parser<&'a str, Output = Self, Error = Error>
    {
        consumed(alt(((space0, line_ending), (space1, eof)))).map(|(segment, _)| Self::new(segment))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod parser {
        use super::*;
        use nom::error::Error;

        macro_rules! failure_case {
            ($test:ident, $segment:expr) => {
                #[test]
                fn $test() {
                    assert!(BlankLineSegment::parser::<Error<&str>>()
                        .parse($segment)
                        .is_err())
                }
            };
        }

        macro_rules! success_case {
            ($test:ident, $segment:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(
                        BlankLineSegment::parser::<Error<&str>>().parse($segment.clone()),
                        Ok(("", BlankLineSegment::new($segment)))
                    )
                }
            };
        }

        failure_case!(should_reject_empty, "");
        failure_case!(should_reject_line_with_a_char, "    a\n");

        success_case!(should_work_with_one_whitespace, " ");
        success_case!(should_work_with_a_single_newline, "\n");
        success_case!(should_work_with_a_single_tab, "\t");
        success_case!(should_work_with_any_whitespace, " \t\r\n");
    }
}
