use crate::{
    ast::block::{BlankLine, LinkReferenceDefinition},
    render::DisplayHtml,
};

impl DisplayHtml for BlankLine<'_> {
    fn display_html(&self, _: &mut String, _: &[LinkReferenceDefinition]) {
        // Blank lines are ignored.
    }
}
