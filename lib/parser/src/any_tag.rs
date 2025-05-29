use super::{Parser, SplitAt, utils::Reverse};
use crate::{ItemsIndices, ParseResult};

pub fn any_tag<Tags, Tag, TagItem>(tags: Tags) -> AnyTagParser<Tags, Tag, TagItem> {
    AnyTagParser::new(tags)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnyTagParser<Tags, Tag, TagItem> {
    tags: Tags,
    _tags_item: std::marker::PhantomData<Tag>,
    _tag_item: std::marker::PhantomData<TagItem>,
}

impl<Tags, Tag, TagItem> AnyTagParser<Tags, Tag, TagItem> {
    fn new(tag: Tags) -> Self {
        Self {
            tags: tag,
            _tags_item: std::marker::PhantomData,
            _tag_item: std::marker::PhantomData,
        }
    }
}

impl<I, Tags, Tag, TagItem> Parser<I> for AnyTagParser<Tags, Tag, TagItem>
where
    I: SplitAt + ItemsIndices<TagItem>,
    Tags: ItemsIndices<Tag>,
    Tag: ItemsIndices<TagItem>,
    TagItem: PartialEq,
{
    type Output = I;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output> {
        for tag in self.tags.items() {
            if let Some(index) = input.after_prefix(tag.items()) {
                return Ok(input.split_at(index).reverse());
            }
        }
        Err(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_reject_with_empty_tags() {
        let tags: [&str; 0] = [];
        assert_eq!(Err("abc"), any_tag(&tags).parse("abc"));
    }

    #[test]
    fn should_reject_if_not_tag_matches() {
        assert_eq!(Err("abc"), any_tag(&["xyz", "1234"]).parse("abc"));
    }

    // Just to show the difference in behavior between an empty
    // collection of tags an an empty tag.
    #[test]
    fn empty_tag_should_match_anything() {
        assert_eq!(Ok(("abc", "")), any_tag(&[""]).parse("abc"));
    }

    #[test]
    fn should_work_if_the_first_tag_matches() {
        assert_eq!(Ok(("c", "ab")), any_tag(&["ab", "1234"]).parse("abc"));
    }

    #[test]
    fn should_work_if_the_second_tag_matches() {
        assert_eq!(
            Ok(("abc", "1234")),
            any_tag(&["ab", "1234"]).parse("1234abc")
        );
    }
}
