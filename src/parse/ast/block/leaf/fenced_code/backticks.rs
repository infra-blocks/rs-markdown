use crate::{
    ast::block::BackticksFencedCode,
    parse::{
        input::Input,
        segment::fenced_code::{
            BackticksFencedCodeClosingSegment, BackticksFencedCodeOpeningSegment,
        },
        traits::Parse,
    },
};
use parser::{ItemsIndices, Map, ParseResult, Parser, Validate};

enum ContentOrClosingSegment<'a> {
    Content(&'a str),
    Closing(BackticksFencedCodeClosingSegment<'a>),
}

fn content_or_closing_segment<'a, I: Input<'a>>(
    opening: &BackticksFencedCodeOpeningSegment<'a>,
) -> impl Fn(I) -> ParseResult<I, ContentOrClosingSegment<'a>> {
    |input: I| {
        if input.is_empty() {
            return Err(input);
        }
        println!("received input: {input:?}");
        match BackticksFencedCodeClosingSegment::parse
            .validate(|segment: &BackticksFencedCodeClosingSegment| segment.closes(opening))
            .map(ContentOrClosingSegment::Closing)
            .parse(input)
        {
            Ok((remaining, closing)) => Ok((remaining, closing)),
            Err(input) => {
                // If it's not a closing segment, then it's content. It's safe to unwrap because we have already
                // checked that the input is not empty.
                let mut enumerator = <I as ItemsIndices<&'a str>>::enumerate(&input);
                let (_, segment) = enumerator.next().unwrap();
                let (_, remaining) = input.split_at(enumerator.next_index());
                Ok((remaining, ContentOrClosingSegment::Content(segment)))
            }
        }
    }
}

impl<'a> Parse<'a> for BackticksFencedCode<'a> {
    fn parse<I: Input<'a>>(input: I) -> ParseResult<I, Self> {
        let (mut remaining, opening) = BackticksFencedCodeOpeningSegment::parse(input)?;
        let mut content_segments = Vec::new();
        loop {
            let result = content_or_closing_segment(&opening).parse(remaining);
            match result {
                Ok((inner, ContentOrClosingSegment::Content(segment))) => {
                    remaining = inner;
                    content_segments.push(segment);
                }
                Ok((inner, ContentOrClosingSegment::Closing(closing_segment))) => {
                    return Ok((
                        inner,
                        Self::new(opening, content_segments, Some(closing_segment)),
                    ));
                }
                Err(input) => {
                    // If we get there it's because we ran out of input.
                    return Ok((input, Self::new(opening, content_segments, None)));
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod parse {
        use super::*;
        use crate::parse::test_utils::{StrictParse, test_parse_macros};

        test_parse_macros!(BackticksFencedCode);

        failure_case!(should_fail_with_empty_string, "");
        failure_case!(should_fail_with_blank_line, "\n");

        success_case!(
            should_work_with_missing_closing_segment,
            "```",
            parsed => BackticksFencedCode::new(
                BackticksFencedCodeOpeningSegment::strict_parse("```"),
                vec![],
                None
            )
        );
        success_case!(
            should_work_without_content,
            "```\n```\n",
            parsed => BackticksFencedCode::new(
                BackticksFencedCodeOpeningSegment::strict_parse("```\n"),
                vec![],
                Some(BackticksFencedCodeClosingSegment::strict_parse("```\n"))
            )
        );
        success_case!(
            should_work_with_content,
            "```\nabc\ndef\n```\n",
            parsed => BackticksFencedCode::new(
                BackticksFencedCodeOpeningSegment::strict_parse("```\n"),
                vec!["abc\n", "def\n"],
                Some(BackticksFencedCodeClosingSegment::strict_parse("```\n"))
            )
        );
        success_case!(
            smaller_closing_fences_should_be_treated_as_content,
            "````\nabc\ndef\n```\n````",
            parsed => BackticksFencedCode::new(
                BackticksFencedCodeOpeningSegment::strict_parse("````\n"),
                vec!["abc\n", "def\n", "```\n"],
                Some(BackticksFencedCodeClosingSegment::strict_parse("````"))
            )
        );
    }
}
