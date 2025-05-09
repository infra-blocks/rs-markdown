mod atx_heading;
mod blank_line;
mod fenced_code;
mod indented_code;
mod link_reference_definition;
mod setext_heading;
mod thematic_break;

use crate::Segments;
pub use atx_heading::*;
pub use blank_line::*;
pub use fenced_code::*;
pub use indented_code::*;
pub use link_reference_definition::*;
pub use setext_heading::*;
use std::iter::FusedIterator;
pub use thematic_break::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Leaf<'a> {
    AtxHeading(AtxHeading<'a>),
    BlankLine(BlankLine<'a>),
    FencedCode(FencedCode<'a>),
    IndentedCode(IndentedCode<'a>),
    LinkReferenceDefinition(LinkReferenceDefinition<'a>),
    ThematicBreak(thematic_break::ThematicBreak<'a>),
}

impl<'a> Segments<'a> for Leaf<'a> {
    type SegmentsIter = LeafSegmentsIterator<'a>;

    fn segments(&'a self) -> Self::SegmentsIter {
        LeafSegmentsIterator::from(self)
    }
}

// TODO: turn into statically typed enum.
pub struct LeafSegmentsIterator<'a> {
    iter: Box<dyn Iterator<Item = &'a str> + 'a>,
}

impl<'a> LeafSegmentsIterator<'a> {
    fn new(iter: Box<dyn Iterator<Item = &'a str> + 'a>) -> Self {
        Self { iter }
    }
}

impl<'a> From<&'a Leaf<'a>> for LeafSegmentsIterator<'a> {
    fn from(leaf: &'a Leaf) -> Self {
        match leaf {
            Leaf::AtxHeading(heading) => Self::new(Box::new(heading.segments())),
            Leaf::BlankLine(blank_line) => Self::new(Box::new(blank_line.segments())),
            Leaf::FencedCode(fenced_code) => Self::new(Box::new(fenced_code.segments())),
            Leaf::IndentedCode(indented_code) => Self::new(Box::new(indented_code.segments())),
            Leaf::LinkReferenceDefinition(_link_reference_definition) => {
                unimplemented!("LinkReferenceDefinition text() not implemented")
            }
            Leaf::ThematicBreak(thematic_break) => Self::new(Box::new(thematic_break.segments())),
        }
    }
}

impl FusedIterator for LeafSegmentsIterator<'_> {}

impl<'a> Iterator for LeafSegmentsIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
