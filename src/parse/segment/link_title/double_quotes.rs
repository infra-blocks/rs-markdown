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
use parser::{ParseResult, Parser, ZeroToMany};
use std::{iter::FusedIterator, slice};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DoubleQuotesLinkTitleSingleSegment<'a>(&'a str);

impl<'a> DoubleQuotesLinkTitleSingleSegment<'a> {
    fn new(segment: &'a str) -> Self {
        Self(segment)
    }
}

impl<'a> NomParse<'a> for DoubleQuotesLinkTitleSingleSegment<'a> {
    fn nom_parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error>
    where
        Self: Sized,
    {
        recognize((tag("\""), utils::valid_characters, tag("\"")))
            .map(Self::new)
            .parse(input)
    }
}

impl<'a> Segment<'a> for DoubleQuotesLinkTitleSingleSegment<'a> {
    fn segment(&self) -> &'a str {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoubleQuotesLinkTitleMultiSegments<'a> {
    opening: DoubleQuotesLinkTitleOpeningSegment<'a>,
    continuations: Vec<DoubleQuotesLinkTitleContinuationSegment<'a>>,
    closing: DoubleQuotesLinkTitleClosingSegment<'a>,
}

impl<'a> DoubleQuotesLinkTitleMultiSegments<'a> {
    fn new(
        opening: DoubleQuotesLinkTitleOpeningSegment<'a>,
        continuations: Vec<DoubleQuotesLinkTitleContinuationSegment<'a>>,
        closing: DoubleQuotesLinkTitleClosingSegment<'a>,
    ) -> Self {
        Self {
            opening,
            continuations,
            closing,
        }
    }
}

impl<'a> Parse<&'a str> for DoubleQuotesLinkTitleMultiSegments<'a> {
    fn parse<I: Input<&'a str>>(input: I) -> ParseResult<I, Self> {
        let (remaining, opening) = DoubleQuotesLinkTitleOpeningSegment::parse(input)?;
        let (remaining, continuations) = DoubleQuotesLinkTitleContinuationSegment::parse
            .zero_to_many()
            .parse(remaining)?;
        let (remaining, closing) = DoubleQuotesLinkTitleClosingSegment::parse(remaining)?;
        Ok((remaining, Self::new(opening, continuations, closing)))
    }
}

impl<'a> Segments<'a> for DoubleQuotesLinkTitleMultiSegments<'a> {
    type SegmentsIter = DoubleQuotesLinkTitleMultiSegmentsIter<'a>;

    fn segments(&'a self) -> Self::SegmentsIter {
        self.into()
    }
}

pub struct DoubleQuotesLinkTitleMultiSegmentsIter<'a> {
    opening: Option<&'a str>,
    continuations: slice::Iter<'a, DoubleQuotesLinkTitleContinuationSegment<'a>>,
    closing: Option<&'a str>,
}

impl<'a> From<&'a DoubleQuotesLinkTitleMultiSegments<'a>>
    for DoubleQuotesLinkTitleMultiSegmentsIter<'a>
{
    fn from(title: &'a DoubleQuotesLinkTitleMultiSegments<'a>) -> Self {
        Self {
            opening: Some(title.opening.0),
            continuations: title.continuations.iter(),
            closing: Some(title.closing.0),
        }
    }
}

impl FusedIterator for DoubleQuotesLinkTitleMultiSegmentsIter<'_> {}

impl<'a> Iterator for DoubleQuotesLinkTitleMultiSegmentsIter<'a> {
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
pub struct DoubleQuotesLinkTitleOpeningSegment<'a>(&'a str);

impl<'a> DoubleQuotesLinkTitleOpeningSegment<'a> {
    fn new(segment: &'a str) -> Self {
        Self(segment)
    }
}

impl<'a> NomParse<'a> for DoubleQuotesLinkTitleOpeningSegment<'a> {
    fn nom_parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error>
    where
        Self: Sized,
    {
        recognize((tag("\""), utils::valid_characters, line_ending))
            .map(Self::new)
            .parse(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DoubleQuotesLinkTitleContinuationSegment<'a>(&'a str);

impl<'a> DoubleQuotesLinkTitleContinuationSegment<'a> {
    fn new(segment: &'a str) -> Self {
        Self(segment)
    }
}

impl<'a> NomParse<'a> for DoubleQuotesLinkTitleContinuationSegment<'a> {
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
pub struct DoubleQuotesLinkTitleClosingSegment<'a>(&'a str);

impl<'a> DoubleQuotesLinkTitleClosingSegment<'a> {
    fn new(segment: &'a str) -> Self {
        Self(segment)
    }
}

impl<'a> NomParse<'a> for DoubleQuotesLinkTitleClosingSegment<'a> {
    fn nom_parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error>
    where
        Self: Sized,
    {
        recognize((utils::valid_characters, tag("\"")))
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

            test_parse_macros!(DoubleQuotesLinkTitleSingleSegment);

            failure_case!(should_reject_empty, "");
            failure_case!(should_reject_single_newline, "\n");
            failure_case!(should_reject_blank_line, " \t\n");
            failure_case!(should_reject_leading_whitespace, r#" """#);
            failure_case!(should_reject_opening_quote_without_closing, "\"");

            success_case!(should_accept_empty_content, "\"\"");
            success_case!(should_accept_some_text, "\"Hello\"");
            success_case!(should_accept_escaped_quotes, "\"Hello, \\\"Bro\\\"\"");
            success_case!(should_accept_any_escape, "\"Hello, \\;World!\"");
            success_case!(
                should_stop_at_terminating_quotes,
                "\"Hello Bro!\"\n",
                "\"Hello Bro!\"",
                "\n"
            );
        }

        mod multi {
            use super::*;

            mod multi_segments {
                use super::*;
                use crate::parse::test_utils::test_parse_macros;

                test_parse_macros!(DoubleQuotesLinkTitleMultiSegments);

                failure_case!(should_reject_empty, "");
                failure_case!(should_reject_missing_closing_segment, "\"Hello!\n");
                failure_case!(
                    should_reject_blank_line_mid_title,
                    "\"Hello,\n \t\nWorld!\""
                );

                success_case!(should_accept_empty_content, "\"\n\"\n", parsed => DoubleQuotesLinkTitleMultiSegments::new(
                DoubleQuotesLinkTitleOpeningSegment::new("\"\n"),
                vec![],
                DoubleQuotesLinkTitleClosingSegment::new("\"")
            ), "\n");
                success_case!(should_accept_opening_and_closing, "\"Hello,\nWorld!\"", parsed => DoubleQuotesLinkTitleMultiSegments::new(
                    DoubleQuotesLinkTitleOpeningSegment::new("\"Hello,\n"),
                    vec![],
                    DoubleQuotesLinkTitleClosingSegment::new("World!\"")
                ));
                success_case!(should_accept_one_continuation, "\"Hello,\nWorld!\n\"", parsed => DoubleQuotesLinkTitleMultiSegments::new(
                    DoubleQuotesLinkTitleOpeningSegment::new("\"Hello,\n"),
                    vec![DoubleQuotesLinkTitleContinuationSegment::new("World!\n")],
                    DoubleQuotesLinkTitleClosingSegment::new("\"")
                ));
                success_case!(should_accept_many_continuations, r#""Hello,
World!
How are you?
Good?
  Goooooooooooooooood  ""#,
                    parsed => DoubleQuotesLinkTitleMultiSegments::new(
                        DoubleQuotesLinkTitleOpeningSegment::new("\"Hello,\n"),
                        vec![
                            DoubleQuotesLinkTitleContinuationSegment::new("World!\n"),
                            DoubleQuotesLinkTitleContinuationSegment::new("How are you?\n"),
                            DoubleQuotesLinkTitleContinuationSegment::new("Good?\n"),
                        ],
                        DoubleQuotesLinkTitleClosingSegment::new("  Goooooooooooooooood  \"")
                    )
                );
                success_case!(should_stop_at_terminating_quotes, r#""Hello,
World"
This is not included!"#,
                    parsed => DoubleQuotesLinkTitleMultiSegments::new(
                        DoubleQuotesLinkTitleOpeningSegment::new("\"Hello,\n"),
                        vec![],
                        DoubleQuotesLinkTitleClosingSegment::new("World\"")
                    ),
                    "\nThis is not included!"
                );
            }

            mod opening {
                use super::*;
                use crate::parse::test_utils::test_parse_macros;

                test_parse_macros!(DoubleQuotesLinkTitleOpeningSegment);

                failure_case!(should_reject_empty, "");
                failure_case!(should_reject_single_newline, "\n");
                failure_case!(should_reject_blank_line, " \t\n");
                failure_case!(should_reject_leading_whitespace, r#" """#);
                // Indeed, there should be more than one line, otherwise we won't close it for sure.
                failure_case!(should_reject_opening_quote_without_newline, r#"""#);

                success_case!(should_accept_single_opening_quote, "\"\n");
                success_case!(should_accept_some_text, "\"Hello,\n");
                success_case!(should_accept_escaped_quotes, "\"Hello, \\\"Bro\\\"\n");
                success_case!(should_accept_any_escape, "\"Hello, \\;World!\n");
            }

            mod continuation {
                use super::*;
                use crate::parse::test_utils::test_parse_macros;

                test_parse_macros!(DoubleQuotesLinkTitleContinuationSegment);

                // For it to be a continuation segment, there mustn't be any unescaped quotes,
                // it must end with a newline, and it cannot be a blank line.
                failure_case!(should_reject_empty, "");
                failure_case!(should_reject_single_newline, "\n");
                failure_case!(should_reject_blank_line, " \t\n");
                failure_case!(should_reject_double_quotes, "\"\n");
                failure_case!(
                    should_reject_missing_newline,
                    "this is not exactly a continuation"
                );

                success_case!(should_accept_a_single_character, "a\n");
                success_case!(should_accept_leading_whitespace, " \ta\n");
                success_case!(should_accept_trailing_whitespace, "a \n");
                success_case!(should_accept_single_quotes, "a'\n");
                success_case!(should_accept_escaped_quotes, "a\\\"b\n");
                success_case!(should_accept_any_escape, "a\\;b\n");
            }

            mod closing {
                use super::*;
                use crate::parse::test_utils::test_parse_macros;

                test_parse_macros!(DoubleQuotesLinkTitleClosingSegment);

                failure_case!(should_reject_empty, "");
                failure_case!(should_reject_single_newline, "\n");
                failure_case!(should_reject_blank_line, " \t\n");
                failure_case!(
                    should_reject_missing_quotes,
                    "Hello is this title closed yet?"
                );
                failure_case!(should_reject_inline_newline, "Hello\nWorld\"");

                success_case!(should_accept_single_closing_quote, "\"");
                success_case!(should_accept_leading_whitespace, " \"");
                success_case!(should_accept_some_text, "Hello\"");
                success_case!(should_accept_escaped_quotes, "\\\"\"");
                success_case!(
                    should_not_include_terminating_newline,
                    "Hello\"\n",
                    "Hello\"",
                    "\n"
                );
                success_case!(
                    should_stop_at_first_closing_quote,
                    "Here is the content\" and ignore that part",
                    "Here is the content\"",
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
                    DoubleQuotesLinkTitleMultiSegments::strict_parse("\"Hello,\nWorld!\"");
                let segments: Vec<_> = link_title.segments().collect();
                assert_eq!(vec!["\"Hello,\n", "World!\""], segments);
            }

            #[test]
            fn should_work_with_continuations() {
                let link_title = DoubleQuotesLinkTitleMultiSegments::strict_parse(
                    "\"Hello,\nWorld!\nIs it me\nYou lookin' fo'?\"",
                );
                let segments: Vec<_> = link_title.segments().collect();
                assert_eq!(
                    vec!["\"Hello,\n", "World!\n", "Is it me\n", "You lookin' fo'?\""],
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
    /// It accepts any escape sequence, but rejects unescaped quotes and terminating backslashes
    /// (without a follow character). It also does not allow new lines or carriage returns. This
    /// logic is expected to be handled outside of this function.
    pub fn valid_characters<'a, Error: ParseError<&'a str>>(
        input: &'a str,
    ) -> IResult<&'a str, &'a str, Error> {
        recognize(many0(alt((escaped_sequence, is_not("\\\"\r\n"))))).parse(input)
    }

    #[cfg(test)]
    mod test {
        use super::*;

        mod valid_characters {
            use super::*;
            use nom::error::Error;

            #[test]
            fn should_not_ingest_double_quotes() {
                let input = "\"";
                let (remaining, parsed) = valid_characters::<Error<&str>>(input).unwrap();
                assert_eq!("\"", remaining);
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
            fn should_ingest_escaped_double_quotes() {
                let input = "\\\"";
                let (remaining, parsed) = valid_characters::<Error<&str>>(input).unwrap();
                assert_eq!("", remaining);
                assert_eq!("\\\"", parsed);
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
            fn should_stop_at_double_quotes() {
                let input = "Hello, \"World!";
                let (remaining, parsed) = valid_characters::<Error<&str>>(input).unwrap();
                assert_eq!("\"World!", remaining);
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
