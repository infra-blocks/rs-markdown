mod paragraph;

use crate::parse::phase_1::block::OpenParagraph;
use paragraph::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenLeaf<'a> {
    Paragraph(OpenParagraph<'a>),
}

impl<'a> OpenLeaf<'a> {
    pub fn is_paragraph(&self) -> bool {
        matches!(self, OpenLeaf::Paragraph(_))
    }
}
