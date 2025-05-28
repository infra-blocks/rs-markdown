pub mod atx_heading;
pub mod blank_line;
pub mod fenced_code;
pub mod html;
pub mod indented_code;
pub mod link_reference_definition;
pub mod thematic_break;

use crate::{
    ast::block::{AtxHeading, BlankLine, FencedCode, Html, IndentedCode, Leaf, ThematicBreak},
    parse::{input::Input, traits::Parse},
};
use parser::{Map, ParseResult, Parser, one_of};

impl<'a> Parse<'a> for Leaf<'a> {
    fn parse<I: Input<'a>>(input: I) -> ParseResult<I, Self> {
        one_of((
            AtxHeading::parse.map(Leaf::AtxHeading),
            BlankLine::parse.map(Leaf::BlankLine),
            FencedCode::parse.map(Leaf::FencedCode),
            Html::parse.map(Leaf::Html),
            IndentedCode::parse.map(Leaf::IndentedCode),
            ThematicBreak::parse.map(Leaf::ThematicBreak),
            // Paragraph should be last.
        ))
        .parse(input)
    }
}
