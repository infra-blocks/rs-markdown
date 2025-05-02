use crate::parse::ast::block::leaf::{
    link_reference_definition::LinkReferenceDefinition, thematic_break::ThematicBreak,
};

use super::DisplayHtml;

impl DisplayHtml for ThematicBreak<'_> {
    fn display_html(&self, buffer: &mut String, _: &[LinkReferenceDefinition]) {
        buffer.push_str("<hr />");
        if self.segment().ends_with('\n') {
            buffer.push('\n');
        }
    }
}
