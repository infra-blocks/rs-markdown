use crate::{
    Segment, Segments,
    parse::{
        input::Input,
        parser_utils::is_blank_line,
        traits::{NomParse, Parse},
    },
};
use nom::{
    IResult, Parser as _,
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::{recognize, verify},
    error::ParseError,
};
use parser::{ParseResult, Parser, Repeated};
use std::{iter::FusedIterator, slice};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParenthesesLinkTitleSingleSegment<'a>(&'a str);

impl<'a> ParenthesesLinkTitleSingleSegment<'a> {
    fn new(segment: &'a str) -> Self {
        Self(segment)
    }
}

impl<'a> NomParse<'a> for ParenthesesLinkTitleSingleSegment<'a> {
    fn nom_parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error>
    where
        Self: Sized,
    {
        recognize((tag("("), utils::valid_characters, tag(")")))
            .map(Self::new)
            .parse(input)
    }
}

impl<'a> Segment<'a> for ParenthesesLinkTitleSingleSegment<'a> {
    fn segment(&self) -> &'a str {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParenthesesLinkTitleMultiSegments<'a> {
    opening: ParenthesesLinkTitleOpeningSegment<'a>,
    continuations: Vec<ParenthesesLinkTitleContinuationSegment<'a>>,
    closing: ParenthesesLinkTitleClosingSegment<'a>,
}

impl<'a> ParenthesesLinkTitleMultiSegments<'a> {
    fn new(
        opening: ParenthesesLinkTitleOpeningSegment<'a>,
        continuations: Vec<ParenthesesLinkTitleContinuationSegment<'a>>,
        closing: ParenthesesLinkTitleClosingSegment<'a>,
    ) -> Self {
        Self {
            opening,
            continuations,
            closing,
        }
    }
}

impl<'a> Parse<&'a str> for ParenthesesLinkTitleMultiSegments<'a> {
    fn parse<I: Input<&'a str>>(input: I) -> ParseResult<I, Self> {
        let (remaining, opening) = ParenthesesLinkTitleOpeningSegment::parse(input)?;
        let (remaining, continuations) = ParenthesesLinkTitleContinuationSegment::parse
            .repeated()
            .parse(remaining)?;
        let (remaining, closing) = ParenthesesLinkTitleClosingSegment::parse(remaining)?;
        Ok((remaining, Self::new(opening, continuations, closing)))
    }
}

impl<'a> Segments<'a> for ParenthesesLinkTitleMultiSegments<'a> {
    type SegmentsIter = ParenthesesLinkTitleMultiSegmentsIter<'a>;

    fn segments(&'a self) -> Self::SegmentsIter {
        self.into()
    }
}

pub struct ParenthesesLinkTitleMultiSegmentsIter<'a> {
    opening: Option<&'a str>,
    continuations: slice::Iter<'a, ParenthesesLinkTitleContinuationSegment<'a>>,
    closing: Option<&'a str>,
}

impl<'a> From<&'a ParenthesesLinkTitleMultiSegments<'a>>
    for ParenthesesLinkTitleMultiSegmentsIter<'a>
{
    fn from(title: &'a ParenthesesLinkTitleMultiSegments<'a>) -> Self {
        Self {
            opening: Some(title.opening.0),
            continuations: title.continuations.iter(),
            closing: Some(title.closing.0),
        }
    }
}

impl FusedIterator for ParenthesesLinkTitleMultiSegmentsIter<'_> {}

impl<'a> Iterator for ParenthesesLinkTitleMultiSegmentsIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(opening) = self.opening.take() {
            return Some(opening);
        }

        if let Some(continuation) = self.continuations.next() {
            return Some(continuation.0);
        }

        self.closing.take()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParenthesesLinkTitleOpeningSegment<'a>(&'a str);

impl<'a> ParenthesesLinkTitleOpeningSegment<'a> {
    fn new(segment: &'a str) -> Self {
        Self(segment)
    }
}

impl<'a> NomParse<'a> for ParenthesesLinkTitleOpeningSegment<'a> {
    fn nom_parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error>
    where
        Self: Sized,
    {
        recognize((tag("("), utils::valid_characters, line_ending))
            .map(Self::new)
            .parse(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParenthesesLinkTitleContinuationSegment<'a>(&'a str);

impl<'a> ParenthesesLinkTitleContinuationSegment<'a> {
    fn new(segment: &'a str) -> Self {
        Self(segment)
    }
}

impl<'a> NomParse<'a> for ParenthesesLinkTitleContinuationSegment<'a> {
    fn nom_parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error>
    where
        Self: Sized,
    {
        verify(
            recognize((utils::valid_characters, line_ending)),
            |output: &str| !is_blank_line(output),
        )
        .map(Self::new)
        .parse(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParenthesesLinkTitleClosingSegment<'a>(&'a str);

impl<'a> ParenthesesLinkTitleClosingSegment<'a> {
    fn new(segment: &'a str) -> Self {
        Self(segment)
    }
}

impl<'a> NomParse<'a> for ParenthesesLinkTitleClosingSegment<'a> {
    fn nom_parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error>
    where
        Self: Sized,
    {
        recognize((utils::valid_characters, tag(")")))
            .map(Self::new)
            .parse(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod parse {
        use super::*;

        mod single {
            use super::*;
            use crate::parse::test_utils::test_parse_macros;

            test_parse_macros!(ParenthesesLinkTitleSingleSegment);

            failure_case!(should_reject_empty, "");
            failure_case!(should_reject_single_newline, "\n");
            failure_case!(should_reject_blank_line, " \t\n");
            failure_case!(should_reject_leading_whitespace, r#" """#);
            failure_case!(should_reject_opening_without_closing, "(");
            failure_case!(should_reject_closing_without_opening, ")");

            success_case!(should_accept_empty_content, "()");
            success_case!(should_accept_some_text, "(Hello)");
            success_case!(should_accept_escaped_parentheses, "(Hello, \\(Bro\\))");
            success_case!(should_accept_any_escape, "(Hello, \\;World!)");
            success_case!(
                should_stop_at_terminating_parentheses,
                "(Hello Bro!)\n",
                "(Hello Bro!)",
                "\n"
            );
        }

        mod multi {
            use super::*;

            mod multi_segments {
                use super::*;
                use crate::parse::test_utils::test_parse_macros;

                test_parse_macros!(ParenthesesLinkTitleMultiSegments);

                failure_case!(should_reject_empty, "");
                failure_case!(should_reject_missing_closing_segment, "(Hello!\n");
                failure_case!(should_reject_blank_line_mid_title, "(Hello,\n \t\nWorld!)");

                success_case!(should_accept_empty_content, "(\n)\n", parsed => ParenthesesLinkTitleMultiSegments::new(
                    ParenthesesLinkTitleOpeningSegment::new("(\n"),
                    vec![],
                    ParenthesesLinkTitleClosingSegment::new(")")
                ), "\n");
                success_case!(should_accept_opening_and_closing, "(Hello,\nWorld!)", parsed => ParenthesesLinkTitleMultiSegments::new(
                    ParenthesesLinkTitleOpeningSegment::new("(Hello,\n"),
                    vec![],
                    ParenthesesLinkTitleClosingSegment::new("World!)")
                ));
                success_case!(should_accept_one_continuation, "(Hello,\nWorld!\n)", parsed => ParenthesesLinkTitleMultiSegments::new(
                    ParenthesesLinkTitleOpeningSegment::new("(Hello,\n"),
                    vec![ParenthesesLinkTitleContinuationSegment::new("World!\n")],
                    ParenthesesLinkTitleClosingSegment::new(")")
                ));
                success_case!(should_accept_many_continuations, r"(Hello,
World!
How are you?
Good?
  Goooooooooooooooood  )",
                    parsed => ParenthesesLinkTitleMultiSegments::new(
                        ParenthesesLinkTitleOpeningSegment::new("(Hello,\n"),
                        vec![
                            ParenthesesLinkTitleContinuationSegment::new("World!\n"),
                            ParenthesesLinkTitleContinuationSegment::new("How are you?\n"),
                            ParenthesesLinkTitleContinuationSegment::new("Good?\n"),
                        ],
                        ParenthesesLinkTitleClosingSegment::new("  Goooooooooooooooood  )")
                    )
                );
                success_case!(should_stop_at_terminating_parenthesis, r"(Hello,
World)
This is not included!",
                    parsed => ParenthesesLinkTitleMultiSegments::new(
                        ParenthesesLinkTitleOpeningSegment::new("(Hello,\n"),
                        vec![],
                        ParenthesesLinkTitleClosingSegment::new("World)")
                    ),
                    "\nThis is not included!"
                );
            }

            mod opening {
                use super::*;
                use crate::parse::test_utils::test_parse_macros;

                test_parse_macros!(ParenthesesLinkTitleOpeningSegment);

                failure_case!(should_reject_empty, "");
                failure_case!(should_reject_single_newline, "\n");
                failure_case!(should_reject_blank_line, " \t\n");
                failure_case!(should_reject_leading_whitespace, " (");
                // Indeed, there should be more than one line, otherwise we won't close it for sure.
                failure_case!(should_reject_opening_parenthesis_without_newline, "(");
                failure_case!(should_reject_closing_parenthesis, "()\n");

                success_case!(should_accept_single_opening_parenthesis, "(\n");
                success_case!(should_accept_some_text, "(Hello,\n");
                success_case!(should_accept_escaped_parentheses, "(Hello, \\(Bro\\)\n");
                success_case!(should_accept_any_escape, "(Hello, \\;World!\n");
            }

            mod continuation {
                use super::*;
                use crate::parse::test_utils::test_parse_macros;

                test_parse_macros!(ParenthesesLinkTitleContinuationSegment);

                // For it to be a continuation segment, there mustn't be any unescaped parentheses,
                // it must end with a newline, and it cannot be a blank line.
                failure_case!(should_reject_empty, "");
                failure_case!(should_reject_single_newline, "\n");
                failure_case!(should_reject_blank_line, " \t\n");
                failure_case!(should_reject_opening_parenthesis, "(\n");
                failure_case!(should_reject_closing_parenthesis, ")\n");
                failure_case!(
                    should_reject_missing_newline,
                    "this is not exactly a continuation"
                );

                success_case!(should_accept_a_single_character, "a\n");
                success_case!(should_accept_leading_whitespace, " \ta\n");
                success_case!(should_accept_trailing_whitespace, "a \n");
                success_case!(should_accept_single_quotes, "a'\n");
                success_case!(should_accept_double_quotes, "a\"\n");
                success_case!(should_accept_escaped_parentheses, "a\\(b\\)\n");
                success_case!(should_accept_any_escape, "a\\;b\n");
            }

            mod closing {
                use super::*;
                use crate::parse::test_utils::test_parse_macros;

                test_parse_macros!(ParenthesesLinkTitleClosingSegment);

                failure_case!(should_reject_empty, "");
                failure_case!(should_reject_single_newline, "\n");
                failure_case!(should_reject_blank_line, " \t\n");
                failure_case!(
                    should_reject_missing_closing_parenthesis,
                    "Hello is this title closed yet?"
                );
                failure_case!(should_reject_inline_newline, "Hello\nWorld)");
                failure_case!(should_reject_opening_parenthesis, "(Hello)");

                success_case!(should_accept_single_closing_parenthesis, ")");
                success_case!(should_accept_leading_whitespace, " )");
                success_case!(should_accept_some_text, "Hello)");
                success_case!(should_accept_escaped_parentheses, "\\))");
                success_case!(
                    should_not_include_terminating_newline,
                    "Hello)\n",
                    "Hello)",
                    "\n"
                );
                success_case!(
                    should_stop_at_first_closing_parenthesis,
                    "Here is the content) and ignore that part",
                    "Here is the content)",
                    " and ignore that part"
                );
            }
        }
    }

    mod segments {
        use super::*;

        mod multi {
            use super::*;
            use crate::parse::test_utils::StrictParse;

            #[test]
            fn should_work_without_continuations() {
                let link_title =
                    ParenthesesLinkTitleMultiSegments::strict_parse("(Hello,\nWorld!)");
                let segments: Vec<_> = link_title.segments().collect();
                assert_eq!(vec!["(Hello,\n", "World!)"], segments);
            }

            #[test]
            fn should_work_with_continuations() {
                let link_title = ParenthesesLinkTitleMultiSegments::strict_parse(
                    "(Hello,\nWorld!\nIs it me\nYou lookin' fo'?)",
                );
                let segments: Vec<_> = link_title.segments().collect();
                assert_eq!(
                    vec!["(Hello,\n", "World!\n", "Is it me\n", "You lookin' fo'?)"],
                    segments
                );
            }
        }
    }
}

mod utils {
    use crate::parse::utils::escaped_sequence;
    use nom::{
        IResult, Parser, branch::alt, bytes::complete::is_not, combinator::recognize,
        error::ParseError, multi::many0,
    };

    /// Parses the input string to extract all characters that valid within a link title segment.
    ///
    /// It accepts any escape sequence, but rejects unescaped parentheses and terminating backslashes
    /// (without a follow character). It also does not allow new lines or carriage returns. This
    /// logic is expected to be handled outside of this function.
    pub fn valid_characters<'a, Error: ParseError<&'a str>>(
        input: &'a str,
    ) -> IResult<&'a str, &'a str, Error> {
        recognize(many0(alt((escaped_sequence, is_not("\\()\r\n"))))).parse(input)
    }

    #[cfg(test)]
    mod test {
        use super::*;

        mod valid_characters {
            use super::*;
            use nom::error::Error;

            #[test]
            fn should_not_ingest_opening_parenthesis() {
                let input = "(";
                let (remaining, parsed) = valid_characters::<Error<&str>>(input).unwrap();
                assert_eq!("(", remaining);
                assert_eq!("", parsed);
            }

            #[test]
            fn should_not_ingest_closing_parenthesis() {
                let input = ")";
                let (remaining, parsed) = valid_characters::<Error<&str>>(input).unwrap();
                assert_eq!(")", remaining);
                assert_eq!("", parsed);
            }

            #[test]
            fn should_not_ingest_backslash() {
                let input = "\\";
                let (remaining, parsed) = valid_characters::<Error<&str>>(input).unwrap();
                assert_eq!("\\", remaining);
                assert_eq!("", parsed);
            }

            #[test]
            fn should_ingest_escaped_opening_parenthesis() {
                let input = "\\(";
                let (remaining, parsed) = valid_characters::<Error<&str>>(input).unwrap();
                assert_eq!("", remaining);
                assert_eq!("\\(", parsed);
            }

            #[test]
            fn should_ingest_escaped_closing_parenthesis() {
                let input = "\\)";
                let (remaining, parsed) = valid_characters::<Error<&str>>(input).unwrap();
                assert_eq!("", remaining);
                assert_eq!("\\)", parsed);
            }

            #[test]
            fn should_ingest_any_escaped_sequence() {
                let input = "\\;";
                let (remaining, parsed) = valid_characters::<Error<&str>>(input).unwrap();
                assert_eq!("", remaining);
                assert_eq!("\\;", parsed);
            }

            #[test]
            fn should_ingest_anything_else() {
                let input = "Hello, World!";
                let (remaining, parsed) = valid_characters::<Error<&str>>(input).unwrap();
                assert_eq!("", remaining);
                assert_eq!("Hello, World!", parsed);
            }

            #[test]
            fn should_stop_at_opening_parenthesis() {
                let input = "Hello, (World!";
                let (remaining, parsed) = valid_characters::<Error<&str>>(input).unwrap();
                assert_eq!("(World!", remaining);
                assert_eq!("Hello, ", parsed);
            }

            #[test]
            fn should_stop_at_closing_parenthesis() {
                let input = "Hello, )World!";
                let (remaining, parsed) = valid_characters::<Error<&str>>(input).unwrap();
                assert_eq!(")World!", remaining);
                assert_eq!("Hello, ", parsed);
            }

            #[test]
            fn should_stop_at_terminating_backslash() {
                let input = "Hello, World!\\";
                let (remaining, parsed) = valid_characters::<Error<&str>>(input).unwrap();
                assert_eq!("\\", remaining);
                assert_eq!("Hello, World!", parsed);
            }
        }
    }
}
