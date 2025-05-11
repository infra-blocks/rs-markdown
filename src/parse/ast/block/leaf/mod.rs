pub mod atx_heading;
pub mod blank_line;
pub mod fenced_code;
pub mod indented_code;
pub mod link_reference_definition;
pub mod setext_heading;
pub mod thematic_break;

use crate::parse::{input::NomParse, traits::Segments};
use atx_heading::AtxHeading;
use blank_line::BlankLine;
use fenced_code::FencedCode;
use indented_code::IndentedCode;
use link_reference_definition::LinkReferenceDefinition;
use nom::{Parser, branch::alt};
use std::iter::FusedIterator;
use thematic_break::ThematicBreak;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Leaf<'a> {
    AtxHeading(AtxHeading<'a>),
    BlankLine(BlankLine<'a>),
    FencedCode(FencedCode<'a>),
    IndentedCode(IndentedCode<'a>),
    LinkReferenceDefinition(LinkReferenceDefinition<'a>),
    ThematicBreak(thematic_break::ThematicBreak<'a>),
}

impl<'a> NomParse<'a> for Leaf<'a> {
    fn nom_parse<Error: nom::error::ParseError<&'a str>>(
        input: &'a str,
    ) -> nom::IResult<&'a str, Self, Error> {
        alt((
            AtxHeading::nom_parse.map(Leaf::AtxHeading),
            BlankLine::nom_parse.map(Leaf::BlankLine),
            FencedCode::nom_parse.map(Leaf::FencedCode),
            IndentedCode::nom_parse.map(Leaf::IndentedCode),
            ThematicBreak::nom_parse.map(Leaf::ThematicBreak),
        ))
        .parse(input)
    }
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
