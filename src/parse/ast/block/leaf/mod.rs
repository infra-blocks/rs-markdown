pub mod atx_heading;
pub mod blank_line;
pub mod fenced_code;
pub mod indented_code;
pub mod thematic_break;

use crate::{
    ast::block::{AtxHeading, BlankLine, FencedCode, IndentedCode, Leaf, ThematicBreak},
    parse::{input::Input, traits::Parse},
};
use parser::{Map, ParseResult, Parser, one_of};

impl<'a> Parse<&'a str> for Leaf<'a> {
    fn parse<I: Input<&'a str>>(input: I) -> ParseResult<I, Self> {
        one_of((
            AtxHeading::parse.map(Leaf::AtxHeading),
            BlankLine::parse.map(Leaf::BlankLine),
            FencedCode::parse.map(Leaf::FencedCode),
            IndentedCode::parse.map(Leaf::IndentedCode),
            ThematicBreak::parse.map(Leaf::ThematicBreak),
        ))
        .parse(input)
    }
}
