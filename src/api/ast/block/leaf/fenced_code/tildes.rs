use crate::{
    Segment, Segments,
    parse::segment::fenced_code::{TildesFencedCodeClosingSegment, TildesFencedCodeOpeningSegment},
};
use std::{iter::FusedIterator, slice};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TildesFencedCode<'a> {
    opening_segment: TildesFencedCodeOpeningSegment<'a>,
    content_segments: Vec<&'a str>,
    /// The closing segment is allowed to be None in one scenario: when the end of input is reached
    /// before a closing segment. This is allowed by the spec.
    closing_segment: Option<TildesFencedCodeClosingSegment<'a>>,
}

impl<'a> TildesFencedCode<'a> {
    pub fn content_segments(&'a self) -> TildesFencedCodeContentSegmentsIterator<'a> {
        self.into()
    }

    pub(crate) fn new(
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
        use crate::parse::test_utils::StrictParse;

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

    mod segments {
        use crate::parse::test_utils::StrictParse;

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
