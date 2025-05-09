use crate::{
    Segment,
    ast::{AtxHeading, LinkReferenceDefinition},
    render::display_html::DisplayHtml,
};

impl AtxHeading<'_> {
    fn display_raw_content(&self, buffer: &mut String) {
        if !self.title().is_empty() {
            buffer.push_str(self.title());
        }
    }

    fn display_end_of_line(&self, buffer: &mut String) {
        if self.segment().ends_with('\n') {
            buffer.push('\n');
        }
    }
}

impl DisplayHtml for AtxHeading<'_> {
    fn display_html(&self, buffer: &mut String, _: &[LinkReferenceDefinition]) {
        // TODO: use the link refs and inline text.
        match self.level() {
            1 => {
                buffer.push_str("<h1>");
                self.display_raw_content(buffer);
                buffer.push_str("</h1>");
                self.display_end_of_line(buffer);
            }
            2 => {
                buffer.push_str("<h2>");
                self.display_raw_content(buffer);
                buffer.push_str("</h2>");
                self.display_end_of_line(buffer);
            }
            3 => {
                buffer.push_str("<h3>");
                self.display_raw_content(buffer);
                buffer.push_str("</h3>");
                self.display_end_of_line(buffer);
            }
            4 => {
                buffer.push_str("<h4>");
                self.display_raw_content(buffer);
                buffer.push_str("</h4>");
                self.display_end_of_line(buffer);
            }
            5 => {
                buffer.push_str("<h5>");
                self.display_raw_content(buffer);
                buffer.push_str("</h5>");
                self.display_end_of_line(buffer);
            }
            6 => {
                buffer.push_str("<h6>");
                self.display_raw_content(buffer);
                buffer.push_str("</h6>");
                self.display_end_of_line(buffer);
            }
            _ => panic!("invalid level {:?}", self.level()),
        }
    }
}
