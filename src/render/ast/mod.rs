mod block;

use super::display_html::DisplayHtml;
use crate::{
    ToHtml, Tree, parse::ast::block::leaf::link_reference_definition::LinkReferenceDefinition,
};

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

impl ToHtml for Tree<'_> {
    fn to_html(&self) -> String {
        let mut buffer = String::new();
        self.display_html(&mut buffer, &self.link_reference_definitions);
        buffer
    }
}
