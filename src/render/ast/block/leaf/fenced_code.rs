use crate::parse::ast::block::leaf::{
    fenced_code::FencedCode, link_reference_definition::LinkReferenceDefinition,
};

use super::DisplayHtml;

trait PushContentSegment {
    fn push_content_segment(&mut self, segment: &str);
}

impl PushContentSegment for String {
    // TODO: add the unindent.
    fn push_content_segment(&mut self, segment: &str) {
        for char in segment.chars() {
            // Escape html chars!
            match char {
                '&' => self.push_str("&amp;"),
                '<' => self.push_str("&lt;"),
                '>' => self.push_str("&gt;"),
                '"' => self.push_str("&quot;"),
                '\'' => self.push_str("&#x27;"),
                '/' => self.push_str("&#x2F;"),
                _ => self.push(char),
            }
        }
    }
}

impl DisplayHtml for FencedCode<'_> {
    fn display_html(
        &self,
        buffer: &mut String,
        _link_reference_definitions: &[LinkReferenceDefinition],
    ) {
        buffer.push_str("<pre><code>");
        for segment in self.content_segments() {
            buffer.push_content_segment(segment);
        }
        buffer.push_str("</code></pre>");
    }
}
