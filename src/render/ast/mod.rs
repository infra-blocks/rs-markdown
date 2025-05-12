mod block;

use super::display_html::DisplayHtml;
use crate::ast::{LinkReferenceDefinition, Tree};

impl DisplayHtml for Tree<'_> {
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
