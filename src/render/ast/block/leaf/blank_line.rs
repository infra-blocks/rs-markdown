use crate::{
    parse::ast::block::leaf::{
        blank_line::BlankLine, link_reference_definition::LinkReferenceDefinition,
    },
    render::display_html::DisplayHtml,
};

impl DisplayHtml for BlankLine<'_> {
    fn display_html(&self, _: &mut String, _: &[LinkReferenceDefinition]) {
        // Blank lines are ignored.
    }
}
