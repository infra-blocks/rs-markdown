use crate::{
    Segment, Segments,
    parse::{
        input::Input,
        parsers::line_ending,
        predicates::is_blank_line,
        traits::{Parse, ParseLine},
    },
};
use parser::{Map, ParseResult, Parser, Repeated, recognize, tag, validate};
use std::{iter::FusedIterator, slice};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SingleQuotesLinkTitleSingleSegment<'a>(&'a str);

impl<'a> SingleQuotesLinkTitleSingleSegment<'a> {
    fn new(segment: &'a str) -> Self {
        Self(segment)
    }
}

impl<'a> ParseLine<'a> for SingleQuotesLinkTitleSingleSegment<'a> {
    fn parse_line(input: &'a str) -> ParseResult<&'a str, Self> {
        recognize((tag("'"), utils::valid_characters, tag("'")))
            .map(Self::new)
            .parse(input)
    }
}

impl<'a> Segment<'a> for SingleQuotesLinkTitleSingleSegment<'a> {
    fn segment(&self) -> &'a str {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SingleQuotesLinkTitleMultiSegments<'a> {
    opening: SingleQuotesLinkTitleOpeningSegment<'a>,
    continuations: Vec<SingleQuotesLinkTitleContinuationSegment<'a>>,
    closing: SingleQuotesLinkTitleClosingSegment<'a>,
}

impl<'a> SingleQuotesLinkTitleMultiSegments<'a> {
    fn new(
        opening: SingleQuotesLinkTitleOpeningSegment<'a>,
        continuations: Vec<SingleQuotesLinkTitleContinuationSegment<'a>>,
        closing: SingleQuotesLinkTitleClosingSegment<'a>,
    ) -> Self {
        Self {
            opening,
            continuations,
            closing,
        }
    }
}

impl<'a> Parse<&'a str> for SingleQuotesLinkTitleMultiSegments<'a> {
    fn parse<I: Input<&'a str>>(input: I) -> ParseResult<I, Self> {
        let (remaining, opening) = SingleQuotesLinkTitleOpeningSegment::parse(input)?;
        let (remaining, continuations) = SingleQuotesLinkTitleContinuationSegment::parse
            .repeated()
            .parse(remaining)?;
        let (remaining, closing) = SingleQuotesLinkTitleClosingSegment::parse(remaining)?;
        Ok((remaining, Self::new(opening, continuations, closing)))
    }
}

impl<'a> Segments<'a> for SingleQuotesLinkTitleMultiSegments<'a> {
    type SegmentsIter = SingleQuotesLinkTitleMultiSegmentsIter<'a>;

    fn segments(&'a self) -> Self::SegmentsIter {
        self.into()
    }
}

pub struct SingleQuotesLinkTitleMultiSegmentsIter<'a> {
    opening: Option<&'a str>,
    continuations: slice::Iter<'a, SingleQuotesLinkTitleContinuationSegment<'a>>,
    closing: Option<&'a str>,
}

impl<'a> From<&'a SingleQuotesLinkTitleMultiSegments<'a>>
    for SingleQuotesLinkTitleMultiSegmentsIter<'a>
{
    fn from(title: &'a SingleQuotesLinkTitleMultiSegments<'a>) -> Self {
        Self {
            opening: Some(title.opening.0),
            continuations: title.continuations.iter(),
            closing: Some(title.closing.0),
        }
    }
}

impl FusedIterator for SingleQuotesLinkTitleMultiSegmentsIter<'_> {}

impl<'a> Iterator for SingleQuotesLinkTitleMultiSegmentsIter<'a> {
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
pub struct SingleQuotesLinkTitleOpeningSegment<'a>(&'a str);

impl<'a> SingleQuotesLinkTitleOpeningSegment<'a> {
    fn new(segment: &'a str) -> Self {
        Self(segment)
    }
}

impl<'a> ParseLine<'a> for SingleQuotesLinkTitleOpeningSegment<'a> {
    fn parse_line(input: &'a str) -> ParseResult<&'a str, Self> {
        recognize((tag("'"), utils::valid_characters, line_ending))
            .map(Self::new)
            .parse(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SingleQuotesLinkTitleContinuationSegment<'a>(&'a str);

impl<'a> SingleQuotesLinkTitleContinuationSegment<'a> {
    fn new(segment: &'a str) -> Self {
        Self(segment)
    }
}

impl<'a> ParseLine<'a> for SingleQuotesLinkTitleContinuationSegment<'a> {
    fn parse_line(input: &'a str) -> ParseResult<&'a str, Self> {
        validate(
            recognize((utils::valid_characters, line_ending)),
            |segment: &&str| !is_blank_line(segment),
        )
        .map(Self::new)
        .parse(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SingleQuotesLinkTitleClosingSegment<'a>(&'a str);

impl<'a> SingleQuotesLinkTitleClosingSegment<'a> {
    fn new(segment: &'a str) -> Self {
        Self(segment)
    }
}

impl<'a> ParseLine<'a> for SingleQuotesLinkTitleClosingSegment<'a> {
    fn parse_line(input: &'a str) -> ParseResult<&'a str, Self> {
        recognize((utils::valid_characters, tag("'")))
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

            test_parse_macros!(SingleQuotesLinkTitleSingleSegment);

            failure_case!(should_reject_empty, "");
            failure_case!(should_reject_single_newline, "\n");
            failure_case!(should_reject_blank_line, " \t\n");
            failure_case!(should_reject_leading_whitespace, r#" """#);
            failure_case!(should_reject_opening_quote_without_closing, "'");

            success_case!(should_accept_empty_content, "''");
            success_case!(should_accept_some_text, "'Hello'");
            success_case!(should_accept_escaped_quotes, "'Hello, \\'Bro\\''");
            success_case!(should_accept_any_escape, "'Hello, \\;World!'");
            success_case!(
                should_stop_at_terminating_quotes,
                "'Hello Bro!'\n",
                "'Hello Bro!'",
                "\n"
            );
        }

        mod multi {
            use super::*;

            mod multi_segments {
                use super::*;
                use crate::parse::test_utils::test_parse_macros;

                test_parse_macros!(SingleQuotesLinkTitleMultiSegments);

                failure_case!(should_reject_empty, "");
                failure_case!(should_reject_missing_closing_segment, "'Hello!\n");
                failure_case!(should_reject_blank_line_mid_title, "'Hello,\n \t\nWorld!'");

                success_case!(should_accept_empty_content, "'\n'\n", parsed => SingleQuotesLinkTitleMultiSegments::new(
                SingleQuotesLinkTitleOpeningSegment::new("'\n"),
                vec![],
                SingleQuotesLinkTitleClosingSegment::new("'")
            ), "\n");
                success_case!(should_accept_opening_and_closing, "'Hello,\nWorld!'", parsed => SingleQuotesLinkTitleMultiSegments::new(
                    SingleQuotesLinkTitleOpeningSegment::new("'Hello,\n"),
                    vec![],
                    SingleQuotesLinkTitleClosingSegment::new("World!'")
                ));
                success_case!(should_accept_one_continuation, "'Hello,\nWorld!\n'", parsed => SingleQuotesLinkTitleMultiSegments::new(
                    SingleQuotesLinkTitleOpeningSegment::new("'Hello,\n"),
                    vec![SingleQuotesLinkTitleContinuationSegment::new("World!\n")],
                    SingleQuotesLinkTitleClosingSegment::new("'")
                ));
                success_case!(should_accept_many_continuations, r"'Hello,
World!
How are you?
Good?
  Goooooooooooooooood  '",
                    parsed => SingleQuotesLinkTitleMultiSegments::new(
                        SingleQuotesLinkTitleOpeningSegment::new("'Hello,\n"),
                        vec![
                            SingleQuotesLinkTitleContinuationSegment::new("World!\n"),
                            SingleQuotesLinkTitleContinuationSegment::new("How are you?\n"),
                            SingleQuotesLinkTitleContinuationSegment::new("Good?\n"),
                        ],
                        SingleQuotesLinkTitleClosingSegment::new("  Goooooooooooooooood  '")
                    )
                );
                success_case!(should_stop_at_terminating_quotes, r"'Hello,
World'
This is not included!",
                    parsed => SingleQuotesLinkTitleMultiSegments::new(
                        SingleQuotesLinkTitleOpeningSegment::new("'Hello,\n"),
                        vec![],
                        SingleQuotesLinkTitleClosingSegment::new("World'")
                    ),
                    "\nThis is not included!"
                );
            }

            mod opening {
                use super::*;
                use crate::parse::test_utils::test_parse_macros;

                test_parse_macros!(SingleQuotesLinkTitleOpeningSegment);

                failure_case!(should_reject_empty, "");
                failure_case!(should_reject_single_newline, "\n");
                failure_case!(should_reject_blank_line, " \t\n");
                failure_case!(should_reject_leading_whitespace, r#" '"#);
                // Indeed, there should be more than one line, otherwise we won't close it for sure.
                failure_case!(should_reject_opening_quote_without_newline, r#"'"#);

                success_case!(should_accept_single_opening_quote, "'\n");
                success_case!(should_accept_some_text, "'Hello,\n");
                success_case!(should_accept_escaped_quotes, "'Hello, \\'Bro\\'\n");
                success_case!(should_accept_any_escape, "'Hello, \\;World!\n");
            }

            mod continuation {
                use super::*;
                use crate::parse::test_utils::test_parse_macros;

                test_parse_macros!(SingleQuotesLinkTitleContinuationSegment);

                // For it to be a continuation segment, there mustn't be any unescaped quotes,
                // it must end with a newline, and it cannot be a blank line.
                failure_case!(should_reject_empty, "");
                failure_case!(should_reject_single_newline, "\n");
                failure_case!(should_reject_blank_line, " \t\n");
                failure_case!(should_reject_single_quotes, "'\n");
                failure_case!(
                    should_reject_missing_newline,
                    "this is not exactly a continuation"
                );

                success_case!(should_accept_a_single_character, "a\n");
                success_case!(should_accept_leading_whitespace, " \ta\n");
                success_case!(should_accept_trailing_whitespace, "a \n");
                success_case!(should_accept_double_quotes, "a\"\n");
                success_case!(should_accept_escaped_quotes, "a\\'b\n");
                success_case!(should_accept_any_escape, "a\\;b\n");
            }

            mod closing {
                use super::*;
                use crate::parse::test_utils::test_parse_macros;

                test_parse_macros!(SingleQuotesLinkTitleClosingSegment);

                failure_case!(should_reject_empty, "");
                failure_case!(should_reject_single_newline, "\n");
                failure_case!(should_reject_blank_line, " \t\n");
                failure_case!(
                    should_reject_missing_quotes,
                    "Hello is this title closed yet?"
                );
                failure_case!(should_reject_inline_newline, "Hello\nWorld'");

                success_case!(should_accept_single_closing_quote, "'");
                success_case!(should_accept_leading_whitespace, " '");
                success_case!(should_accept_some_text, "Hello'");
                success_case!(should_accept_escaped_quotes, "\\''");
                success_case!(
                    should_not_include_terminating_newline,
                    "Hello'\n",
                    "Hello'",
                    "\n"
                );
                success_case!(
                    should_stop_at_first_closing_quote,
                    "Here is the content' and ignore that part",
                    "Here is the content'",
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
                    SingleQuotesLinkTitleMultiSegments::strict_parse("'Hello,\nWorld!'");
                let segments: Vec<_> = link_title.segments().collect();
                assert_eq!(vec!["'Hello,\n", "World!'"], segments);
            }

            #[test]
            fn should_work_with_continuations() {
                let link_title = SingleQuotesLinkTitleMultiSegments::strict_parse(
                    "'Hello,\nWorld!\nIs it me\nYou lookin\\' fo\\'?'",
                );
                let segments: Vec<_> = link_title.segments().collect();
                assert_eq!(
                    vec![
                        "'Hello,\n",
                        "World!\n",
                        "Is it me\n",
                        "You lookin\\' fo\\'?'"
                    ],
                    segments
                );
            }
        }
    }
}

mod utils {
    use crate::parse::parsers::escaped_sequence;
    use parser::{ParseResult, Parser, is_one_of, not, one_of, recognize, repeated, take_while};

    /// Parses the input string to extract all characters that are valid within a link title segment.
    ///
    /// It accepts any escape sequence, but rejects unescaped quotes and terminating backslashes
    /// (without a follow character). It also does not allow new lines or carriage returns. This
    /// logic is expected to be handled outside of this function.
    pub fn valid_characters(input: &str) -> ParseResult<&str, &str> {
        recognize(repeated(one_of((
            escaped_sequence,
            take_while(not(is_one_of(&['\\', '\'', '\r', '\n']))).at_least(1),
        ))))
        .parse(input)
    }

    #[cfg(test)]
    mod test {
        use super::*;

        mod valid_characters {
            use super::*;

            #[test]
            fn should_not_ingest_single_quotes() {
                let input = "'";
                let (remaining, parsed) = valid_characters(input).unwrap();
                assert_eq!("'", remaining);
                assert_eq!("", parsed);
            }

            #[test]
            fn should_not_ingest_backslash() {
                let input = "\\";
                let (remaining, parsed) = valid_characters(input).unwrap();
                assert_eq!("\\", remaining);
                assert_eq!("", parsed);
            }

            #[test]
            fn should_ingest_escaped_single_quotes() {
                let input = "\\'";
                let (remaining, parsed) = valid_characters(input).unwrap();
                assert_eq!("", remaining);
                assert_eq!("\\'", parsed);
            }

            #[test]
            fn should_ingest_any_escaped_sequence() {
                let input = "\\;";
                let (remaining, parsed) = valid_characters(input).unwrap();
                assert_eq!("", remaining);
                assert_eq!("\\;", parsed);
            }

            #[test]
            fn should_ingest_anything_else() {
                let input = "Hello, World!";
                let (remaining, parsed) = valid_characters(input).unwrap();
                assert_eq!("", remaining);
                assert_eq!("Hello, World!", parsed);
            }

            #[test]
            fn should_stop_at_single_quotes() {
                let input = "Hello, 'World!";
                let (remaining, parsed) = valid_characters(input).unwrap();
                assert_eq!("'World!", remaining);
                assert_eq!("Hello, ", parsed);
            }

            #[test]
            fn should_stop_at_terminating_backslash() {
                let input = "Hello, World!\\";
                let (remaining, parsed) = valid_characters(input).unwrap();
                assert_eq!("\\", remaining);
                assert_eq!("Hello, World!", parsed);
            }
        }
    }
}
