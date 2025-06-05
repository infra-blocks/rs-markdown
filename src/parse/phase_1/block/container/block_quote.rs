use crate::{
    ast::block::Block,
    parse::{
        parsers::indented_by_less_than_4,
        phase_1::{
            block::OpenBlock,
            traits::{IOpenBlock, Staging},
        },
    },
};
use parser::{ParseResult, Parser, maybe, recognize, tag};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct OpenBlockQuote<'a> {
    staging: Staging<'a>,
    closed: Vec<Block<'a>>,
    current: Option<Box<OpenBlock<'a>>>,
}

impl<'a> OpenBlockQuote<'a> {
    fn current(&self) -> Option<&OpenBlock<'a>> {
        self.current.as_deref()
    }

    fn set_current(&mut self, block: OpenBlock<'a>) {
        assert_eq!(self.current, None);
        self.current = Some(Box::new(block));
    }
}

impl<'a> IOpenBlock<'a> for OpenBlockQuote<'a> {
    fn stage(&mut self, line: &'a str) -> Result<&'a str, ()> {
        match block_quote_marker(line) {
            Ok((remaining, parsed)) => {
                self.staging.set(parsed);
                Ok(remaining)
            }
            Err(_) => Err(()),
        }
    }

    // TODO: containers are allowed "empty commits", since they can survive lazy paragraph continuation lines.
    fn commit(&mut self) {
        if self.staging.has_line_staged() {
            self.staging.reset();
        }
    }

    // TODO: close current?
    fn close(self) -> Block<'a> {
        self.into()
    }
}

fn block_quote_marker(input: &str) -> ParseResult<&str, &str> {
    recognize((indented_by_less_than_4, tag(">"), maybe(tag(" ")))).parse(input)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockQuote<'a> {
    children: Vec<Block<'a>>,
}

impl<'a> BlockQuote<'a> {
    pub fn new(children: Vec<Block<'a>>) -> Self {
        BlockQuote { children }
    }
}

impl<'a> From<OpenBlockQuote<'a>> for BlockQuote<'a> {
    fn from(open: OpenBlockQuote<'a>) -> Self {
        Self::new(open.closed)
    }
}

impl<'a> From<BlockQuote<'a>> for Block<'a> {
    fn from(block_quote: BlockQuote<'a>) -> Self {
        Self::Container(block_quote)
    }
}

impl<'a> From<OpenBlockQuote<'a>> for Block<'a> {
    fn from(open: OpenBlockQuote<'a>) -> Self {
        let mut block = Block::new_quote();
        if let Some(current) = open.current {
            block.append_block(current.close());
        }
        for closed in open.closed {
            block.append_block(closed);
        }
        block
    }
}
