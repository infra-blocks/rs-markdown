use crate::parse::blocks::block::macros::{html_case_macro, open_module_macro};

// This is case 2 in the spec, and covers lines with the following
// start and end conditions:
// - Start condition: line begins with the string <!--.
// - End condition: line contains the string -->.
html_case_macro!(HtmlCase2);

open_module_macro!(
    crate::parse::blocks::block::leaf::html::case_2,
    HtmlCase2,
    "<!--",
    "-->"
);

mod parse {
    use crate::parse::blocks::block::macros::parse::{
        closing_module_macro, content_or_closing_module_macro, opening_maybe_closing_module_macro,
        opening_module_macro,
    };

    pub use closing::*;
    pub use content_or_closing::*;
    pub use opening::*;
    pub use opening_maybe_closing::*;

    opening_module_macro!(
        // TODO: parse::is to transform a parser into a predicate?
        |line: &str| {
            use crate::parse::parsers::indented_by_less_than_4;
            use parser::{Parser, tag};

            (indented_by_less_than_4, tag("<!--")).parse(line).is_ok()
        },
        ["<!--"],
        "<?"
    );
    closing_module_macro!(|line: &str| line.contains("-->"), ["-->"], "?>");

    opening_maybe_closing_module_macro!("<!--\n", "<!---->\n");
    content_or_closing_module_macro!("word\n", "-->\n");
}
