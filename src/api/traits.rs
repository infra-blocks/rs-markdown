use std::iter;

/// This trait is for struct can be represented as a single segment.
///
/// Those can be blocks that are single segment by definition, such as [crate::parse::ast::block::leaf::atx_heading::AtxHeading],
/// or straight up segment types, such as [crate::parse::segment::atx_heading::AtxHeadingSegment].
pub trait Segment<'a> {
    /// Returns the single segment of the block.
    fn segment(&self) -> &'a str;
}

/// This trait is for structs that can be represented as multiple segments.
///
/// This is useful for blocks that are made of multiple segments, such as [crate::parse::ast::block::leaf::fenced_code::FencedCode].
/// A blanket implementation is provided for structs that implement [Segment].
pub trait Segments<'a> {
    type SegmentsIter: Iterator<Item = &'a str>;

    /// Returns the segments of the block.
    fn segments(&'a self) -> Self::SegmentsIter;
}

impl<'a, T> Segments<'a> for T
where
    T: Segment<'a>,
{
    type SegmentsIter = iter::Once<&'a str>;

    fn segments(&self) -> Self::SegmentsIter {
        iter::once(self.segment())
    }
}

/// Produces an HTML string from a reference to the implementer.
pub trait ToHtml {
    /// Produce a valid HTML string from this instance.
    fn to_html(&self) -> String;
}
