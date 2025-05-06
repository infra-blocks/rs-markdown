use crate::parse::{
    segment::fenced_code::{TildesFencedCodeClosingSegment, TildesFencedCodeOpeningSegment},
    traits::{Parse, Segment, Segments},
    utils::line,
};
use nom::{combinator::recognize, error::ParseError, IResult, Parser};
use std::{iter::FusedIterator, slice};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TildesFencedCode<'a> {
    pub opening_segment: TildesFencedCodeOpeningSegment<'a>,
    pub content_segments: Vec<&'a str>,
    /// The closing segment is allowed to be None in one scenario: when the end of input is reached
    /// before a closing segment. This is allowed by the spec.
    pub closing_segment: Option<TildesFencedCodeClosingSegment<'a>>,
}

impl<'a> TildesFencedCode<'a> {
    pub fn content_segments(&'a self) -> TildesFencedCodeContentSegmentsIterator<'a> {
        self.into()
    }

    fn new(
        opening_segment: TildesFencedCodeOpeningSegment<'a>,
        content_segments: Vec<&'a str>,
        closing_segment: Option<TildesFencedCodeClosingSegment<'a>>,
    ) -> Self {
        Self {
            opening_segment,
            content_segments,
            closing_segment,
        }
    }
}

impl<'a> Parse<'a> for TildesFencedCode<'a> {
    fn parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error> {
        let (mut remaining, opening_segment) =
            TildesFencedCodeOpeningSegment::parse::<Error>(input)?;
        let mut content_segments = Vec::new();
        // We then loop until we find a closing segment for the opening segment or the end of input.
        let (remaining, closing_segment) = loop {
            let Ok((inner, closing_segment)) =
                TildesFencedCodeClosingSegment::parse::<Error>(remaining)
            // If it's not a closing segment, we just add it to the content.
            else {
                // Take the line. and count it as content segment.
                match recognize(line::<Error>).parse(remaining) {
                    Ok((inner, line)) => {
                        content_segments.push(line);
                        remaining = inner;
                        continue;
                    }
                    Err(_) => {
                        // If we can't parse a line, we are done.
                        assert_eq!(remaining, "");
                        break ("", None);
                    }
                }
            };
            // If it is a closing segment, we still need to check that its fence length is long enough.
            if closing_segment.closes(&opening_segment) {
                // It's a match for the opening segment, so we are done.
                break (inner, Some(closing_segment));
            } else {
                // Otherwise, we treat it as regular content.
                content_segments.push(closing_segment.segment());
                remaining = inner;
                continue;
            }
        };

        Ok((
            remaining,
            Self::new(opening_segment, content_segments, closing_segment),
        ))
    }
}

impl<'a> Segments<'a> for TildesFencedCode<'a> {
    type SegmentsIter = TildesFencedCodeSegmentsIterator<'a>;

    fn segments(&'a self) -> Self::SegmentsIter {
        self.into()
    }
}

pub struct TildesFencedCodeContentSegmentsIterator<'a> {
    content_segments: slice::Iter<'a, &'a str>,
}

impl<'a> From<&'a TildesFencedCode<'a>> for TildesFencedCodeContentSegmentsIterator<'a> {
    fn from(fenced_code: &'a TildesFencedCode<'a>) -> Self {
        Self {
            content_segments: fenced_code.content_segments.iter(),
        }
    }
}

impl FusedIterator for TildesFencedCodeContentSegmentsIterator<'_> {}

impl<'a> Iterator for TildesFencedCodeContentSegmentsIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.content_segments.next().map(|segment| &**segment)
    }
}

pub struct TildesFencedCodeSegmentsIterator<'a> {
    opening_segment: Option<TildesFencedCodeOpeningSegment<'a>>,
    content_segments: TildesFencedCodeContentSegmentsIterator<'a>,
    closing_segment: Option<TildesFencedCodeClosingSegment<'a>>,
}

impl<'a> From<&'a TildesFencedCode<'a>> for TildesFencedCodeSegmentsIterator<'a> {
    fn from(fenced_code: &'a TildesFencedCode<'a>) -> Self {
        Self {
            opening_segment: Some(fenced_code.opening_segment.clone()),
            content_segments: fenced_code.into(),
            closing_segment: fenced_code.closing_segment.clone(),
        }
    }
}

impl FusedIterator for TildesFencedCodeSegmentsIterator<'_> {}

impl<'a> Iterator for TildesFencedCodeSegmentsIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(opening_segment) = self.opening_segment.take() {
            return Some(opening_segment.segment());
        }
        if let Some(content_segment) = self.content_segments.next() {
            return Some(content_segment);
        }
        if let Some(closing_segment) = self.closing_segment.take() {
            return Some(closing_segment.segment());
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod content_segments {
        use crate::parse::traits::StrictParse;

        use super::*;

        #[test]
        fn should_work_without_content() {
            let fenced_code = TildesFencedCode::strict_parse("~~~\n");
            let content_segments: Vec<&str> = fenced_code.content_segments().collect();
            assert!(content_segments.is_empty());
        }

        #[test]
        fn should_work_without_closing_segment() {
            let fenced_code = TildesFencedCode::strict_parse("~~~\nabc\ndef");
            let content_segments: Vec<&str> = fenced_code.content_segments().collect();
            assert_eq!(content_segments, vec!["abc\n", "def"]);
        }

        #[test]
        fn should_work_with_a_full_block() {
            let fenced_code = TildesFencedCode::strict_parse("~~~\nabc\ndef\n~~~\n");
            let content_segments: Vec<&str> = fenced_code.content_segments().collect();
            assert_eq!(content_segments, vec!["abc\n", "def\n"]);
        }
    }

    mod parse {
        use crate::parse::traits::StrictParse;

        use super::*;
        use nom::error::Error;

        macro_rules! failure_case {
            ($test:ident, $segment:expr) => {
                #[test]
                fn $test() {
                    assert!(TildesFencedCode::parse::<Error<&str>>($segment).is_err());
                }
            };
        }

        macro_rules! success_case {
            ($test:ident, $segment:expr, $expected:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(
                        TildesFencedCode::parse::<Error<&str>>($segment),
                        Ok(("", $expected))
                    );
                }
            };
        }

        failure_case!(should_fail_with_empty_string, "");
        failure_case!(should_fail_with_blank_line, "\n");

        success_case!(
            should_work_with_missing_closing_segment,
            "~~~",
            TildesFencedCode::new(
                TildesFencedCodeOpeningSegment::strict_parse("~~~"),
                vec![],
                None
            )
        );
        success_case!(
            should_work_without_content,
            "~~~\n~~~\n",
            TildesFencedCode::new(
                TildesFencedCodeOpeningSegment::strict_parse("~~~\n"),
                vec![],
                Some(TildesFencedCodeClosingSegment::strict_parse("~~~\n"))
            )
        );
        success_case!(
            should_work_with_content,
            "~~~\nabc\ndef\n~~~\n",
            TildesFencedCode::new(
                TildesFencedCodeOpeningSegment::strict_parse("~~~\n"),
                vec!["abc\n", "def\n"],
                Some(TildesFencedCodeClosingSegment::strict_parse("~~~\n"))
            )
        );
        success_case!(
            smaller_closing_fences_should_be_treated_as_content,
            "~~~~\nabc\ndef\n~~~\n~~~~",
            TildesFencedCode::new(
                TildesFencedCodeOpeningSegment::strict_parse("~~~~\n"),
                vec!["abc\n", "def\n", "~~~\n"],
                Some(TildesFencedCodeClosingSegment::strict_parse("~~~~"))
            )
        );
    }

    mod segments {
        use crate::parse::traits::StrictParse;

        use super::*;

        #[test]
        fn should_work_with_single_opening_segment() {
            let fenced_code = TildesFencedCode::strict_parse("~~~\n");
            let segments: Vec<&str> = fenced_code.segments().collect();
            assert_eq!(segments, vec!["~~~\n"]);
        }

        #[test]
        fn should_work_with_content_segments() {
            let fenced_code = TildesFencedCode::strict_parse("~~~\nabc\ndef\nghi");
            let segments: Vec<&str> = fenced_code.segments().collect();
            assert_eq!(segments, vec!["~~~\n", "abc\n", "def\n", "ghi"]);
        }

        #[test]
        fn should_work_with_a_full_block() {
            let fenced_code = TildesFencedCode::strict_parse("~~~\nabc\ndef\n~~~\n");
            let segments: Vec<&str> = fenced_code.segments().collect();
            assert_eq!(segments, vec!["~~~\n", "abc\n", "def\n", "~~~\n"]);
        }
    }
}
