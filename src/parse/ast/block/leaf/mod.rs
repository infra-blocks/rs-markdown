pub mod atx_heading;
pub mod blank_line;
pub mod link_reference_definition;
pub mod thematic_break;

use atx_heading::AtxHeading;
use blank_line::BlankLine;
use link_reference_definition::LinkReferenceDefinition;
use nom::{branch::alt, Parser};
use thematic_break::ThematicBreak;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Leaf<'a> {
    AtxHeading(AtxHeading<'a>),
    BlankLine(BlankLine<'a>),
    LinkReferenceDefinition(LinkReferenceDefinition<'a>),
    ThematicBreak(thematic_break::ThematicBreak<'a>),
}

impl<'a> Leaf<'a> {
    pub fn parser<Error: nom::error::ParseError<&'a str>>(
    ) -> impl nom::Parser<&'a str, Output = Self, Error = Error> {
        alt((
            AtxHeading::parser().map(Leaf::AtxHeading),
            BlankLine::parser().map(Leaf::BlankLine),
            ThematicBreak::parser().map(Leaf::ThematicBreak),
        ))
    }
}
