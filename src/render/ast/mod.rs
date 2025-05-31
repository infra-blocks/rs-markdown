mod block;

use super::display_html::DisplayHtml;
use crate::ast::{Document, block::LinkReferenceDefinition};

impl DisplayHtml for Document<'_> {
    fn display_html(
        &self,
        buffer: &mut String,
        link_reference_definitions: &[LinkReferenceDefinition],
    ) {
        // TODO: use some form of fold or sum'
        for block in &self.blocks {
            block.display_html(buffer, link_reference_definitions);
        }
    }
}
