use crate::{Segments, ast::block::IndentedCode, render::DisplayHtml};

trait PushUnindentedStr {
    fn push_unindented_str(&mut self, text: &str);
}

impl PushUnindentedStr for String {
    // Unindents the given text by up to 4 spaces.
    // Note: although the spec mentions that tabs are equivalent to 4 spaces of indent,
    // the example [here](https://spec.commonmark.org/0.31.2/#example-2) shows that
    // it is treated as *up to* 4 spaces and not literally 4 spaces as far as indented code goes.
    // If it were exactly 4 spaces, the resulting text in the example would start with 2 spaces
    // (2 + 4 for the tab - 4), but it doesn't have any spaces at the beginning.
    fn push_unindented_str(&mut self, text: &str) {
        let mut trim_index = 0;
        for (index, character) in text.char_indices().take(4) {
            if !character.is_whitespace() {
                break;
            }
            if character == '\t' {
                trim_index = index + 1;
                break;
            }
            if character == ' ' {
                trim_index = index + 1;
            }
        }
        self.push_str(&text[trim_index..]);
    }
}

impl DisplayHtml for IndentedCode<'_> {
    fn display_html(&self, buffer: &mut String, _: &[super::LinkReferenceDefinition]) {
        buffer.push_str("<pre><code>");
        // There should be at least one segment in the block, otherwise, it was improperly constructed.
        for segment in self.segments() {
            buffer.push_unindented_str(segment);
        }
        buffer.push_str("</code></pre>");
    }
}
