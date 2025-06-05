#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenParagraph<'a> {
    segments: Vec<&'a str>,
}
