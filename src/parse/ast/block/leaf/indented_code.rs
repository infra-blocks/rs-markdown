use crate::{
    ast::IndentedCode,
    parse::{
        input::Input,
        parser::ParseResult,
        segment::indented_code::{ContinuationSegments, IndentedCodeSegment},
        traits::Parse,
    },
};

impl<'a> Parse<&'a str> for IndentedCode<'a> {
    fn parse<I: Input<&'a str>>(input: I) -> ParseResult<I, Self> {
        let (remaining, opening_segment) = IndentedCodeSegment::parse(input)?;
        match ContinuationSegments::parse(remaining) {
            Ok((remaining, continuation_segments)) => {
                let indented_code =
                    IndentedCode::multi_segments(opening_segment, continuation_segments);
                Ok((remaining, indented_code))
            }
            Err(remaining) => {
                // If there are no continuation segments, we just return the opening segment.
                let indented_code = IndentedCode::single_segment(opening_segment);
                Ok((remaining, indented_code))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Tests that it properly strips off trailing blank lines when present.
    mod parse {
        use super::*;
        use crate::{
            ast::BlankLine,
            parse::{
                segment::indented_code::IndentedCodeOrBlankLineSegment,
                test_utils::{StrictParse, test_parse_macros},
            },
        };

        test_parse_macros!(IndentedCode);

        failure_case!(should_fail_with_empty_string, "");
        failure_case!(should_fail_with_blank_line, " \n");
        failure_case!(
            should_fail_with_3_spaces,
            "   This is not exactly indented code.\n"
        );

        success_case!(
            should_work_with_one_indented_code_segment,
            "    This is indented code\nThis is not.",
            parsed => IndentedCode::single_segment(
                IndentedCodeSegment::strict_parse("    This is indented code\n")
            ),
            "This is not."
        );
        success_case!(
            should_work_with_tab_indent,
            "\tThis is indented code\nThis is not.",
            parsed => IndentedCode::single_segment(
                IndentedCodeSegment::strict_parse("\tThis is indented code\n")
            ),
            "This is not."
        );
        success_case!(
            should_work_with_several_indentations,
            "  \t  \tThis is indented code\nThis is not.",
            parsed => IndentedCode::single_segment(
                IndentedCodeSegment::strict_parse("  \t  \tThis is indented code\n")
            ),
            "This is not."
        );
        success_case!(
            should_strip_off_trailing_blank_line,
            "    This is indented code\n \n",
            parsed => IndentedCode::single_segment(
                IndentedCodeSegment::strict_parse("    This is indented code\n")
            ),
            " \n"
        );
        success_case!(
            should_work_with_blank_lines_interleaved,
            r"    This is indented code.
 
    That blank line is part of the block too.
 
    And so was that one.
 
But not this one.",
            parsed => IndentedCode::multi_segments(
                IndentedCodeSegment::strict_parse("    This is indented code.\n"),
                ContinuationSegments::new(
                    vec![
                        IndentedCodeOrBlankLineSegment::from(
                            BlankLine::strict_parse(" \n")
                        ),
                        IndentedCodeOrBlankLineSegment::from(
                            IndentedCodeSegment::strict_parse(
                                "    That blank line is part of the block too.\n"
                            )
                        ),
                        IndentedCodeOrBlankLineSegment::from(
                            BlankLine::strict_parse(" \n")
                        )
                    ],
                    IndentedCodeSegment::strict_parse("    And so was that one.\n")
                )
            ),
            " \nBut not this one."
        );
    }
}
