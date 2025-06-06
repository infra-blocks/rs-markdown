mod block;
mod document;
mod open_block;

pub use document::Document;

pub fn parse(input: &str) -> Document<'_> {
    // This parse implementation should first accumulate all the blocks.
    unimplemented!("document parse unimplemented");
    // Then do a second pass to morph paragraphs into link reference definitions
    // and/or setext headings.
}
