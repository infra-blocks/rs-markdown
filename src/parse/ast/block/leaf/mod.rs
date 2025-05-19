pub mod atx_heading;
pub mod blank_line;
pub mod fenced_code;
pub mod indented_code;
pub mod thematic_break;

use crate::{
    ast::{AtxHeading, BlankLine, FencedCode, IndentedCode, Leaf, ThematicBreak},
    parse::{
        ParseResult,
        input::Input,
        parser::{Map, Parser, one_of},
        traits::Parse,
    },
};

impl<'a> Parse<&'a str> for Leaf<'a> {
    fn parse<I: Input<Item = &'a str>>(input: I) -> ParseResult<I, Self> {
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
