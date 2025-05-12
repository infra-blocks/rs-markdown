use crate::{
    ast::IndentedCode,
    parse::{
        segment::indented_code::{ContinuationSegments, IndentedCodeSegment},
        traits::Parse,
    },
};
use nom::{IResult, error::ParseError};

impl<'a> Parse<'a> for IndentedCode<'a> {
    fn parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error> {
        let (remaining, opening_segment) = IndentedCodeSegment::parse::<Error>(input)?;
        match ContinuationSegments::parse::<Error>(remaining) {
            Ok((remaining, continuation_segments)) => {
                let indented_code =
                    IndentedCode::multi_segments(opening_segment, continuation_segments);
                Ok((remaining, indented_code))
            }
            Err(_) => {
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
                test_utils::test_parse_macros, traits::ParseWhole,
            },
        };
        use nom::error::Error;

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
                IndentedCodeSegment::parse_whole::<Error<&str>>("    This is indented code\n")
                    .unwrap()
            ),
            "This is not."
        );
        success_case!(
            should_work_with_tab_indent,
            "\tThis is indented code\nThis is not.",
            parsed => IndentedCode::single_segment(
                IndentedCodeSegment::parse_whole::<Error<&str>>("\tThis is indented code\n")
                    .unwrap()
            ),
            "This is not."
        );
        success_case!(
            should_work_with_several_indentations,
            "  \t  \tThis is indented code\nThis is not.",
            parsed => IndentedCode::single_segment(
                IndentedCodeSegment::parse_whole::<Error<&str>>("  \t  \tThis is indented code\n")
                    .unwrap()
            ),
            "This is not."
        );
        success_case!(
            should_strip_off_trailing_blank_line,
            "    This is indented code\n \n",
            parsed => IndentedCode::single_segment(
                IndentedCodeSegment::parse_whole::<Error<&str>>("    This is indented code\n")
                    .unwrap()
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
                IndentedCodeSegment::parse_whole::<Error<&str>>("    This is indented code.\n")
                    .unwrap(),
                ContinuationSegments::new(
                    vec![
                        IndentedCodeOrBlankLineSegment::from(
                            BlankLine::parse_whole::<Error<&str>>(" \n").unwrap()
                        ),
                        IndentedCodeOrBlankLineSegment::from(
                            IndentedCodeSegment::parse_whole::<Error<&str>>(
                                "    That blank line is part of the block too.\n"
                            )
                            .unwrap()
                        ),
                        IndentedCodeOrBlankLineSegment::from(
                            BlankLine::parse_whole::<Error<&str>>(" \n").unwrap()
                        )
                    ],
                    IndentedCodeSegment::parse_whole::<Error<&str>>("    And so was that one.\n")
                        .unwrap(),
                )
            ),
            " \nBut not this one."
        );
    }
}
