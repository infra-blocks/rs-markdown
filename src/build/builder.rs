use crate::ast::block::Block;

pub enum BuildFlow {
    /// Indicates that the current [IBlockBuilder] was unable to process the line. This means that the caller
    /// should call [IBlockBuilder::close] next, and then retry the line with the next [IBlockBuilder].
    CloseAndRetryLine,
    /// Indicates that the current [IBlockBuilder] has consumed the line and [IBlockBuilder::close] should be called
    /// called next.
    Close,
    /// Indicates that the current [IBlockBuilder] has consumed the line but may consume more lines.
    Continue,
}

pub trait IBlockBuilder<'a>: Sized {
    fn maybe_open(line: &'a str) -> Option<Self>;
    fn parse_line(&mut self, line: &'a str) -> BuildFlow;
    fn close<E: Extend<Block<'a>>>(self, sink: &mut E);
}
