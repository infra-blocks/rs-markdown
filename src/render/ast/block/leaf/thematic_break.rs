use super::DisplayHtml;
use crate::{
    Segment,
    ast::{LinkReferenceDefinition, ThematicBreak},
};

impl DisplayHtml for ThematicBreak<'_> {
    fn display_html(&self, buffer: &mut String, _: &[LinkReferenceDefinition]) {
        buffer.push_str("<hr />");
        if self.segment().ends_with('\n') {
            buffer.push('\n');
        }
    }
}
