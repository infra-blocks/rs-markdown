use crate::{ast::BlankLine, parse::traits::Parse};
use nom::{
    Parser,
    branch::alt,
    character::complete::{line_ending, space0, space1},
    combinator::{consumed, eof},
    error::ParseError,
};

impl<'a> Parse<'a> for BlankLine<'a> {
    fn parse<Error: ParseError<&'a str>>(input: &'a str) -> nom::IResult<&'a str, Self, Error> {
        consumed(alt(((space0, line_ending), (space1, eof))))
            .map(|(segment, _)| Self::new(segment))
            .parse(input)
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
                    assert!(BlankLine::parse::<Error<&str>>($segment).is_err())
                }
            };
        }

        macro_rules! success_case {
            ($test:ident, $segment:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(
                        BlankLine::parse::<Error<&str>>($segment.clone()),
                        Ok(("", BlankLine::new($segment)))
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
