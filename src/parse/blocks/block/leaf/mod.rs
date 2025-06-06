mod atx_heading;
mod blank_line;
mod fenced_code;
mod html;
mod indented_code;
mod link_reference_definition;
mod paragraph;
mod setext_heading;
mod thematic_break;

pub use atx_heading::*;
pub use blank_line::*;
pub use fenced_code::*;
pub use html::*;
pub use indented_code::*;
pub use link_reference_definition::*;
pub use paragraph::*;
pub use setext_heading::*;
pub use thematic_break::*;

use crate::parse::blocks::open_block::IBlock;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Leaf<'a> {
    AtxHeading(AtxHeading<'a>),
    BlankLine(BlankLine<'a>),
    FencedCode(FencedCode<'a>),
    Html(Html<'a>),
    IndentedCode(IndentedCode<'a>),
    LinkReferenceDefinition(LinkReferenceDefinition<'a>),
    Paragraph(Paragraph<'a>),
    SetextHeading(SetextHeading<'a>),
    ThematicBreak(ThematicBreak<'a>),
}

impl<'a> IBlock<'a> for Leaf<'a> {
    type Open = open::Leaf<'a>;

    fn open(line: &'a str) -> parser::ParseResult<&'a str, Self::Open> {
        if let Ok((remaining, atx_heading)) = AtxHeading::open(line) {
            return Ok((remaining, open::Leaf::AtxHeading(atx_heading)));
        }
        if let Ok((remaining, blank_line)) = BlankLine::open(line) {
            return Ok((remaining, open::Leaf::BlankLine(blank_line)));
        }
        if let Ok((remaining, fenced_code)) = ThematicBreak::open(line) {
            return Ok((remaining, open::Leaf::ThematicBreak(fenced_code)));
        }
        panic!("block opening not implemented for line: {}", line);
    }
}

pub mod open {
    use crate::parse::blocks::{
        block::leaf::{
            atx_heading::open::AtxHeading, blank_line::open::BlankLine,
            thematic_break::open::ThematicBreak,
        },
        open_block::IOpenBlock,
    };

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Leaf<'a> {
        AtxHeading(AtxHeading<'a>),
        BlankLine(BlankLine<'a>),
        ThematicBreak(ThematicBreak<'a>),
    }

    impl<'a> IOpenBlock<'a> for Leaf<'a> {
        type Closed = super::Leaf<'a>;

        fn stage(&mut self, line: &'a str) -> Result<&'a str, ()> {
            match self {
                Leaf::AtxHeading(atx_heading) => atx_heading.stage(line),
                Leaf::BlankLine(blank_line) => blank_line.stage(line),
                Leaf::ThematicBreak(thematic_break) => thematic_break.stage(line),
            }
        }

        fn commit(&mut self) {
            match self {
                Leaf::AtxHeading(atx_heading) => atx_heading.commit(),
                Leaf::BlankLine(blank_line) => blank_line.commit(),
                Leaf::ThematicBreak(thematic_break) => thematic_break.commit(),
            }
        }

        fn close(self) -> Self::Closed {
            match self {
                Leaf::AtxHeading(atx_heading) => super::Leaf::AtxHeading(atx_heading.close()),
                Leaf::BlankLine(blank_line) => super::Leaf::BlankLine(blank_line.close()),
                Leaf::ThematicBreak(thematic_break) => {
                    super::Leaf::ThematicBreak(thematic_break.close())
                }
            }
        }
    }
}
