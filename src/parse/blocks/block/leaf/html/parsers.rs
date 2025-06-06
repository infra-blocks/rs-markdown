use parser::{
    IsEmpty, ItemsIndices, ParseResult, Parser, SplitAt, SubsetRange, recognize, take, take_while,
};

/// A tag name consists of an ASCII letter followed by zero or more ASCII letters, digits, or hyphens (-).
pub fn tag_name<I>(input: I) -> ParseResult<I, I>
where
    I: SubsetRange<I> + SplitAt + Clone + IsEmpty + ItemsIndices<char>,
{
    recognize((
        take(1).that(|c: char| c.is_ascii_alphabetic()),
        take_while(|c: char| c.is_ascii_alphanumeric() || c.is_ascii_digit() || c == '-'),
    ))
    .parse(input)
}
