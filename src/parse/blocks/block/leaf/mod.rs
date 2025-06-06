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
        if let Ok((remaining, fenced_code)) = FencedCode::open(line) {
            return Ok((remaining, open::Leaf::FencedCode(fenced_code)));
        }
        if let Ok((remaining, indented_code)) = IndentedCode::open(line) {
            return Ok((remaining, open::Leaf::IndentedCode(indented_code)));
        }
        if let Ok((remaining, thematic_break)) = ThematicBreak::open(line) {
            return Ok((remaining, open::Leaf::ThematicBreak(thematic_break)));
        }
        panic!("block opening not implemented for line: {}", line);
    }
}

impl<'a> From<AtxHeading<'a>> for Leaf<'a> {
    fn from(atx_heading: AtxHeading<'a>) -> Self {
        Self::AtxHeading(atx_heading)
    }
}

impl<'a> From<BlankLine<'a>> for Leaf<'a> {
    fn from(blank_line: BlankLine<'a>) -> Self {
        Self::BlankLine(blank_line)
    }
}

impl<'a> From<FencedCode<'a>> for Leaf<'a> {
    fn from(fenced_code: FencedCode<'a>) -> Self {
        Self::FencedCode(fenced_code)
    }
}

impl<'a> From<Html<'a>> for Leaf<'a> {
    fn from(html: Html<'a>) -> Self {
        Self::Html(html)
    }
}

impl<'a> From<IndentedCode<'a>> for Leaf<'a> {
    fn from(indented_code: IndentedCode<'a>) -> Self {
        Self::IndentedCode(indented_code)
    }
}

impl<'a> From<LinkReferenceDefinition<'a>> for Leaf<'a> {
    fn from(link_reference_definition: LinkReferenceDefinition<'a>) -> Self {
        Self::LinkReferenceDefinition(link_reference_definition)
    }
}

impl<'a> From<Paragraph<'a>> for Leaf<'a> {
    fn from(paragraph: Paragraph<'a>) -> Self {
        Self::Paragraph(paragraph)
    }
}

impl<'a> From<SetextHeading<'a>> for Leaf<'a> {
    fn from(setext_heading: SetextHeading<'a>) -> Self {
        Self::SetextHeading(setext_heading)
    }
}

impl<'a> From<ThematicBreak<'a>> for Leaf<'a> {
    fn from(thematic_break: ThematicBreak<'a>) -> Self {
        Self::ThematicBreak(thematic_break)
    }
}

pub mod open {
    use crate::parse::blocks::{
        block::leaf::{
            atx_heading::open::AtxHeading, blank_line::open::BlankLine,
            fenced_code::open::FencedCode, indented_code::open::IndentedCode,
            thematic_break::open::ThematicBreak,
        },
        open_block::IOpenBlock,
    };

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Leaf<'a> {
        AtxHeading(AtxHeading<'a>),
        BlankLine(BlankLine<'a>),
        FencedCode(FencedCode<'a>),
        IndentedCode(IndentedCode<'a>),
        ThematicBreak(ThematicBreak<'a>),
    }

    impl<'a> IOpenBlock<'a> for Leaf<'a> {
        type Closed = super::Leaf<'a>;

        fn stage(&mut self, line: &'a str) -> Result<&'a str, ()> {
            match self {
                Leaf::AtxHeading(atx_heading) => atx_heading.stage(line),
                Leaf::BlankLine(blank_line) => blank_line.stage(line),
                Leaf::FencedCode(fenced_code) => fenced_code.stage(line),
                Leaf::IndentedCode(indented_code) => indented_code.stage(line),
                Leaf::ThematicBreak(thematic_break) => thematic_break.stage(line),
            }
        }

        fn commit(&mut self) {
            match self {
                Leaf::AtxHeading(atx_heading) => atx_heading.commit(),
                Leaf::BlankLine(blank_line) => blank_line.commit(),
                Leaf::FencedCode(fenced_code) => fenced_code.commit(),
                Leaf::IndentedCode(indented_code) => indented_code.commit(),
                Leaf::ThematicBreak(thematic_break) => thematic_break.commit(),
            }
        }

        fn close<F: FnMut(Self::Closed) -> ()>(self, mut sink: F) {
            match self {
                Leaf::AtxHeading(atx_heading) => {
                    atx_heading.close(|atx_heading| sink(atx_heading.into()))
                }
                Leaf::BlankLine(blank_line) => {
                    blank_line.close(|blank_line| sink(blank_line.into()))
                }
                Leaf::FencedCode(fenced_code) => {
                    fenced_code.close(|fenced_code| sink(fenced_code.into()))
                }
                Leaf::IndentedCode(indented_code) => indented_code.close(|result| {
                    sink(result.indented_code.into());
                    for blank_line in result.blank_lines {
                        sink(blank_line.into());
                    }
                }),
                Leaf::ThematicBreak(thematic_break) => thematic_break
                    .close(|thematic_break| sink(super::Leaf::ThematicBreak(thematic_break))),
            }
        }
    }
}
