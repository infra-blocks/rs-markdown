use crate::{
    ast::block::{Block, BlockQuote},
    build::{
        BuildFlow, IBlockBuilder,
        block::{BlockBuilder, LeafBuilder, paragraph_interrupt_segment},
    },
    parse::ast::block::container::block_quote::marker,
};
use parser::is;
use std::iter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockQuoteBuilder<'a> {
    segments: Vec<&'a str>,
    current: Option<Box<BlockBuilder<'a>>>,
    closed: Vec<Block<'a>>,
}

impl<'a> BlockQuoteBuilder<'a> {
    pub fn new(lines: Vec<&'a str>, current: Option<Box<BlockBuilder<'a>>>) -> Self {
        Self {
            segments: lines,
            current,
            closed: Vec::new(),
        }
    }
}

impl<'a> IBlockBuilder<'a> for BlockQuoteBuilder<'a> {
    fn maybe_open(line: &'a str) -> Option<Self> {
        let (remaining, _) = marker(line).ok()?;
        // TODO: we keep the whole line or just the parsed portion? DUNNO
        let lines = vec![line];
        let current = BlockBuilder::maybe_open(remaining);
        Some(Self::new(lines, current.map(Box::new)))
    }

    fn parse_line(&mut self, line: &'a str) -> BuildFlow {
        match marker(line) {
            Ok((remaining, _)) => {
                // Whatever happens next, we know for a fact this line is part of the block quote,
                self.segments.push(line);
                // If we can parse the marker we dispatch to the current block.
                if let Some(mut builder) = self.current.take() {
                    match builder.parse_line(remaining) {
                        BuildFlow::CloseAndRetryLine => {
                            builder.close(&mut self.closed);
                            // We handle the rest after the if block.
                        }
                        BuildFlow::Close => {
                            builder.close(&mut self.closed);
                            return BuildFlow::Continue;
                        }
                        BuildFlow::Continue => {
                            return BuildFlow::Continue;
                        }
                    }
                }
                // If we make it here it's either because we didn't have a block when parsing the line or
                // we don't have a block anymore as it rejected the line and has been closed. In
                // both cases, it means the line hasn't been consumed yet and so we need to try a new
                // block builder.
                match BlockBuilder::maybe_open(remaining) {
                    Some(builder) => {
                        self.current = Some(Box::new(builder));
                        BuildFlow::Continue
                    }
                    None => BuildFlow::Close,
                }
            }
            Err(_) => {
                // If we can't parse the marker, we can't close yet since it depends if the line can
                // be consumed by an ongoing paragraph.
                let Some(builder) = self.current.take() else {
                    // If we don't have a current block, then for sure we are violating the lazy continuation
                    // line rule, so this block is done and ready to be closed.
                    return BuildFlow::CloseAndRetryLine;
                };

                // If the current block is a container, then
                // we delegate the decision to it.
                if let BlockBuilder::Container(mut container) = *builder {
                    match container.parse_line(line) {
                        BuildFlow::CloseAndRetryLine => {
                            return BuildFlow::CloseAndRetryLine;
                        }
                        BuildFlow::Close => {
                            self.segments.push(line);
                            container.close(&mut self.closed);
                            return BuildFlow::Continue;
                        }
                        BuildFlow::Continue => {
                            self.segments.push(line);
                            // TODO: is there a way to avoid this new boxing? Probably by delegating the above
                            self.current = Some(Box::new(container.into()));
                            return BuildFlow::Continue;
                        }
                    }
                };

                if let BlockBuilder::Leaf(LeafBuilder::Paragraph(mut paragraph)) = *builder {
                    if is(paragraph_interrupt_segment)(line) {
                        // TODO: all of these closures could be postponed until this builder's close method.
                        paragraph.close(&mut self.closed);
                        return BuildFlow::CloseAndRetryLine;
                    }
                    // Otherwise, we expect the inner paragraph to handle the line.
                    self.segments.push(line);
                    match paragraph.parse_line(line) {
                        BuildFlow::CloseAndRetryLine => {
                            panic!(
                                "unexpected paragraph close request after checking for interrupt segments"
                            );
                        }
                        BuildFlow::Close => {
                            paragraph.close(&mut self.closed);
                            return BuildFlow::Continue;
                        }
                        BuildFlow::Continue => {
                            self.current = Some(Box::new(paragraph.into()));
                            return BuildFlow::Continue;
                        }
                    }
                };

                BuildFlow::CloseAndRetryLine
            }
        }
    }

    fn close<E: Extend<Block<'a>>>(self, sink: &mut E) {
        let mut closed = self.closed;
        let current = self.current;
        if let Some(builder) = current {
            builder.close(&mut closed);
        }
        sink.extend(iter::once(BlockQuote::new(self.segments).into()));
    }
}
