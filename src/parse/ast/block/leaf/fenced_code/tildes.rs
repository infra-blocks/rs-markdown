use crate::{
    Segment,
    ast::TildesFencedCode,
    parse::{
        segment::fenced_code::{TildesFencedCodeClosingSegment, TildesFencedCodeOpeningSegment},
        traits::Parse,
        utils::line,
    },
};
use nom::{IResult, Parser, combinator::recognize, error::ParseError};

impl<'a> Parse<'a> for TildesFencedCode<'a> {
    fn parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error> {
        let (mut remaining, opening_segment) =
            TildesFencedCodeOpeningSegment::parse::<Error>(input)?;
        let mut content_segments = Vec::new();
        // We then loop until we find a closing segment for the opening segment or the end of input.
        let (remaining, closing_segment) = loop {
            let Ok((inner, closing_segment)) =
                TildesFencedCodeClosingSegment::parse::<Error>(remaining)
            // If it's not a closing segment, we just add it to the content.
            else {
                // Take the line. and count it as content segment.
                match recognize(line::<Error>).parse(remaining) {
                    Ok((inner, line)) => {
                        content_segments.push(line);
                        remaining = inner;
                        continue;
                    }
                    Err(_) => {
                        // If we can't parse a line, we are done.
                        assert_eq!(remaining, "");
                        break ("", None);
                    }
                }
            };
            // If it is a closing segment, we still need to check that its fence length is long enough.
            if closing_segment.closes(&opening_segment) {
                // It's a match for the opening segment, so we are done.
                break (inner, Some(closing_segment));
            } else {
                // Otherwise, we treat it as regular content.
                content_segments.push(closing_segment.segment());
                remaining = inner;
                continue;
            }
        };

        Ok((
            remaining,
            Self::new(opening_segment, content_segments, closing_segment),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod parse {
        use super::*;
        use crate::parse::{test_utils::test_parse_macros, traits::StrictParse};

        test_parse_macros!(TildesFencedCode);

        failure_case!(should_fail_with_empty_string, "");
        failure_case!(should_fail_with_blank_line, "\n");

        success_case!(
            should_work_with_missing_closing_segment,
            "~~~",
            parsed => TildesFencedCode::new(
                TildesFencedCodeOpeningSegment::strict_parse("~~~"),
                vec![],
                None
            )
        );
        success_case!(
            should_work_without_content,
            "~~~\n~~~\n",
            parsed => TildesFencedCode::new(
                TildesFencedCodeOpeningSegment::strict_parse("~~~\n"),
                vec![],
                Some(TildesFencedCodeClosingSegment::strict_parse("~~~\n"))
            )
        );
        success_case!(
            should_work_with_content,
            "~~~\nabc\ndef\n~~~\n",
            parsed => TildesFencedCode::new(
                TildesFencedCodeOpeningSegment::strict_parse("~~~\n"),
                vec!["abc\n", "def\n"],
                Some(TildesFencedCodeClosingSegment::strict_parse("~~~\n"))
            )
        );
        success_case!(
            smaller_closing_fences_should_be_treated_as_content,
            "~~~~\nabc\ndef\n~~~\n~~~~",
            parsed => TildesFencedCode::new(
                TildesFencedCodeOpeningSegment::strict_parse("~~~~\n"),
                vec!["abc\n", "def\n", "~~~\n"],
                Some(TildesFencedCodeClosingSegment::strict_parse("~~~~"))
            )
        );
    }
}
