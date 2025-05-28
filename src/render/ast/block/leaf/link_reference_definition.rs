use crate::{ast::block::LinkReferenceDefinition, render::DisplayHtml};

impl DisplayHtml for LinkReferenceDefinition<'_> {
    fn display_html(&self, _: &mut String, _: &[LinkReferenceDefinition]) {
        // Link reference definitions are not rendered.
    }
}
