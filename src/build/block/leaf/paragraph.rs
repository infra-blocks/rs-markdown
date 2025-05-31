use crate::{
    build::{BuildFlow, IBlockBuilder},
    parse::{
        ast::block::{
            container::block_quote,
            leaf::{blank_line, html, thematic_break},
        },
        parsers::indented_by_less_than_4,
        segment::{
            fenced_code::fenced_code_opening_segment, setext_heading::setext_heading_underline,
        },
    },
};
use parser::{ParseResult, Parser, is, recognize, rest};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParagraphBuilder<'a> {
    segments: Vec<&'a str>,
    is_setext: bool,
}

impl<'a> ParagraphBuilder<'a> {
    pub(crate) fn new(segments: Vec<&'a str>) -> Self {
        Self {
            segments,
            is_setext: false,
        }
    }
}

/// Most lines are accepted as paragraph opening segments,
/// that's because most of the logic to determine what block a segment
/// belongs to is expected to happen outside. For example, a fenced
/// code opening segment could be accepted by this function. The
/// caller needs to check first if the segment is a fenced code
/// before calling this function.
fn paragraph_opening_segment(input: &str) -> ParseResult<&str, &str> {
    recognize((indented_by_less_than_4, rest)).parse(input)
}

/// Unlike other blocks, pargraphs are aware of how some other block
/// segments are parsed, as they can interupt the its own parsing flow.
/// We chose to make these interruption chekcs here, as opposed to
/// in the caller, since it simplifies the code (each container block
/// magically inherit that logic without having to check if the current
/// block is a paragraph, then test the segment against all possible
/// interruptions). It also makes sense given that the paragraph builder
/// needs to know about setext headings minimally to efficiently morph
/// the ongoing paragraph into one.
///
/// A paragraph can be interrupted by:
///  - Blank lines
///  - Thematic breaks
///  - Atx Headings
///  - A fenced code block.
///  - HTML blocks from type 1-6
///  - Block quotes
///  - The first list item of a list, [under certain conditions](https://spec.commonmark.org/0.31.2/#list-items).
pub fn paragraph_interrupt_segment(segment: &str) -> ParseResult<&str, &str> {
    /// Those interrupt the paragraph without being part of it.
    if is(blank_line::blank_line)(segment)
        || is(thematic_break::thematic_break)(segment)
        || is(fenced_code_opening_segment)(segment)
        || is(html::case_1_opening_segment)(segment)
        || is(html::case_2_opening_segment)(segment)
        || is(html::case_3_opening_segment)(segment)
        || is(html::case_4_opening_segment)(segment)
        || is(html::case_5_opening_segment)(segment)
        || is(html::case_6_opening_segment)(segment)
        || is(block_quote::marker)(segment)
    // TODO: the rule about the list item. It's complicated...
    {
        return rest(segment);
    }
    Err(segment)
}

impl<'a> IBlockBuilder<'a> for ParagraphBuilder<'a> {
    fn maybe_open(line: &'a str) -> Option<Self> {
        if is(paragraph_opening_segment)(line) {
            return Some(Self::new(vec![line]));
        }
        None
    }

    fn parse_line(&mut self, line: &'a str) -> BuildFlow {
        if is(setext_heading_underline)(line) {
            self.is_setext = true;
            self.segments.push(line);
            // We return an error here to signify to the caller that the paragraph won't
            // be taking further lines. However, we return the input *without* the setext
            // heading.
            return BuildFlow::Close;
        }
        // If the line interrupts the paragraph, we return without consuming it.
        if is(paragraph_interrupt_segment)(line) {
            return BuildFlow::CloseAndRetryLine;
        }
        // Otherwise, it's a regular segment.
        self.segments.push(line);
        return BuildFlow::Continue;
    }

    fn close<E: Extend<crate::ast::block::Block<'a>>>(self, sink: &mut E) {
        // The closing of the paragraph is the most complex closing of all blocks.
        // It starts by reparsing the segments from the beginning to create as
        // many [LinkReferenceDefinition]s as possible. The remainder becomes
        // the paragraph text. If the paragraph text has more than one line
        // and the last line is a setext heading underline, it may later end up
        // being a [SetextHeading] block, depending on the context.
        unimplemented!()
    }
}
