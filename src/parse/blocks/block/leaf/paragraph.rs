/// Parsing rules:
/// - A paragraph can be interrupted by:
///   - Thematic breaks
///   - Atx Headings
///   - A fenced code block.
///   - HTML blocks from type 1-6
///   - Block quotes
///   - The first list item of a list, [under certain conditions](https://spec.commonmark.org/0.31.2/#list-items).
/// - A paragraph cannot be interrupted by:
///   - Indented code blocks
///   - A setext heading
///   - HTML blocks of type 7 cannot interrupt a paragraph.
///   - A link reference definition
///   - An empty list item
///
/// A paragraph that ends with a setext heading underline is a setext heading, not a paragraph.
///
/// This example:
/// [label]: /destination
/// =====================
///
/// Results in a link reference definition followed by a paragraph line containing the setext heading underline
/// as tested in the commonmark dingus. However, the setext heading underline is *not* part of a setext heading
/// since there are no preceding text lines. This is because the parsing happened like this:
/// - Take paragraph lines until a blank line or a setext heading underline is found.
/// - Rewind the paragraph content and extract the leading sequence of link reference definitions, if any.
/// - The remainder is the paragraph. *Because the remainder is empty*, a setext heading cannot be formed.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Paragraph<'a> {
    segments: Vec<&'a str>,
}
