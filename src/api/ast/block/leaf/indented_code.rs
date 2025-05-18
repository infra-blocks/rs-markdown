use crate::{
    Segment, Segments,
    parse::segment::indented_code::{ContinuationSegments, IndentedCodeSegment},
};
use std::iter::{self, FusedIterator};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndentedCode<'a> {
    opening_segment: IndentedCodeSegment<'a>,
    continuation_segments: Option<ContinuationSegments<'a>>,
}

impl<'a> IndentedCode<'a> {
    pub(crate) fn new(
        opening_segment: IndentedCodeSegment<'a>,
        continuation_segments: Option<ContinuationSegments<'a>>,
    ) -> Self {
        Self {
            opening_segment,
            continuation_segments,
        }
    }

    pub(crate) fn single_segment(opening_segment: IndentedCodeSegment<'a>) -> Self {
        Self::new(opening_segment, None)
    }

    pub(crate) fn multi_segments(
        opening_segment: IndentedCodeSegment<'a>,
        continuation_segments: ContinuationSegments<'a>,
    ) -> Self {
        Self::new(opening_segment, Some(continuation_segments))
    }
}

impl<'a> Segments<'a> for IndentedCode<'a> {
    type SegmentsIter = IndentedCodeSegmentsIterator<'a>;

    fn segments(&'a self) -> Self::SegmentsIter {
        IndentedCodeSegmentsIterator::from(self)
    }
}

pub struct IndentedCodeSegmentsIterator<'a> {
    opening_segment: Option<&'a str>,
    continuation_segments: Box<dyn Iterator<Item = &'a str> + 'a>,
    closing_segment: Option<&'a str>,
}

impl<'a> IndentedCodeSegmentsIterator<'a> {
    fn new(
        opening_segment: &'a str,
        continuation_segments: Box<dyn Iterator<Item = &'a str> + 'a>,
        closing_segment: Option<&'a str>,
    ) -> Self {
        Self {
            opening_segment: Some(opening_segment),
            continuation_segments,
            closing_segment,
        }
    }
}

impl<'a> From<&'a IndentedCode<'a>> for IndentedCodeSegmentsIterator<'a> {
    fn from(indented_code: &'a IndentedCode<'a>) -> Self {
        let opening_segment = indented_code.opening_segment.segment();
        match &indented_code.continuation_segments {
            None => Self::new(opening_segment, Box::new(iter::empty()), None),
            Some(continuation_segments) => {
                let closing_segment = continuation_segments.closing_segment.segment();
                let continuation_segments = continuation_segments
                    .segments
                    .iter()
                    .map(|segment| segment.segment());
                Self::new(
                    opening_segment,
                    Box::new(continuation_segments),
                    Some(closing_segment),
                )
            }
        }
    }
}

impl FusedIterator for IndentedCodeSegmentsIterator<'_> {}

impl<'a> Iterator for IndentedCodeSegmentsIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(segment) = self.opening_segment.take() {
            return Some(segment);
        }
        if let Some(segment) = self.continuation_segments.next() {
            return Some(segment);
        }
        self.closing_segment.take()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod segments {
        use super::*;
        use crate::parse::traits::StrictParse;
        use std::vec;

        #[test]
        fn should_work_with_single_segment() {
            let indented_code = IndentedCode::strict_parse("    This is indented code\n");
            let segments = indented_code.segments().collect::<Vec<_>>();
            assert_eq!(segments, vec!["    This is indented code\n"]);
        }

        #[test]
        fn should_work_with_multiple_segments() {
            let indented_code = IndentedCode::strict_parse(
                r"    This is indented code

    This is the closing segment.",
            );
            let segments = indented_code.segments().collect::<Vec<_>>();
            assert_eq!(
                segments,
                vec![
                    "    This is indented code\n",
                    "\n",
                    "    This is the closing segment."
                ]
            );
        }
    }
}
