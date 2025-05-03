pub mod atx_heading;
pub mod blank_line;
pub mod indented_code;
pub mod link_reference_definition;
pub mod thematic_break;

use atx_heading::AtxHeading;
use blank_line::BlankLine;
use indented_code::IndentedCode;
use link_reference_definition::LinkReferenceDefinition;
use nom::{branch::alt, Parser};
use thematic_break::ThematicBreak;

use crate::parse::traits::Parse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Leaf<'a> {
    AtxHeading(AtxHeading<'a>),
    BlankLine(BlankLine<'a>),
    IndentedCode(IndentedCode<'a>),
    LinkReferenceDefinition(LinkReferenceDefinition<'a>),
    ThematicBreak(thematic_break::ThematicBreak<'a>),
}

impl<'a> Parse<'a> for Leaf<'a> {
    fn parse<Error: nom::error::ParseError<&'a str>>(
        input: &'a str,
    ) -> nom::IResult<&'a str, Self, Error> {
        alt((
            AtxHeading::parse.map(Leaf::AtxHeading),
            BlankLine::parse.map(Leaf::BlankLine),
            IndentedCode::parse.map(Leaf::IndentedCode),
            ThematicBreak::parse.map(Leaf::ThematicBreak),
        ))
        .parse(input)
    }
}
