use crate::parse::{
    traits::{Parse, Segment},
    utils::{indented_by_less_than_4, is_char, line},
};
use nom::{
    IResult, Parser,
    bytes::complete::take_while1,
    character::complete::space0,
    combinator::{eof, recognize},
    error::ParseError,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetextHeadingEqualsUnderlineSegment<'a>(&'a str);

impl<'a> SetextHeadingEqualsUnderlineSegment<'a> {
    fn new(segment: &'a str) -> Self {
        Self(segment)
    }

    pub fn level(&self) -> u8 {
        1
    }
}

impl<'a> Parse<'a> for SetextHeadingEqualsUnderlineSegment<'a> {
    fn parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error>
    where
        Self: Sized,
    {
        recognize(line.and_then((
            indented_by_less_than_4,
            take_while1(is_char('=')),
            space0,
            eof,
        )))
        .map(Self::new)
        .parse(input)
    }
}

impl<'a> Segment<'a> for SetextHeadingEqualsUnderlineSegment<'a> {
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
                        SetextHeadingEqualsUnderlineSegment::parse::<Error<&str>>($segment)
                            .is_err()
                    );
                }
            };
        }

        macro_rules! success_case {
            ($test:ident, $segment:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(
                        SetextHeadingEqualsUnderlineSegment::parse::<Error<&str>>($segment),
                        Ok(("", SetextHeadingEqualsUnderlineSegment::new($segment)))
                    );
                }
            };
        }

        failure_case!(should_fail_with_empty, "");
        failure_case!(should_fail_with_blank_line, "\n");
        failure_case!(should_fail_with_4_idents, "    =\n");
        failure_case!(should_fail_with_tab_ident, "\t=\n");
        failure_case!(should_reject_trailing_characters, "=a\n");
        failure_case!(should_fail_for_hyphens, "-\n");

        success_case!(should_work_with_single_equal, "=\n");
        success_case!(should_work_without_eol, "=");
        success_case!(should_work_with_5_equals, "=====\n");
        success_case!(should_work_with_3_idents, "   =\n");
        success_case!(should_work_with_trailing_whitespace, "=  \n");
    }
}
