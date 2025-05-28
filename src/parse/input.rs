use parser::{IsEmpty, ItemsIndices, SplitAt, SubsetRange};
use std::{fmt::Debug, iter::Map};

/// A trait to regroup different all the different requirements to use all parser functionalities.
pub trait Input<'a>
where
    Self: ItemsIndices<&'a str>
        + ItemsIndices<char>
        + SubsetRange<Self>
        + SubsetRange<&'a str>
        + SplitAt
        + Clone
        + Debug
        + IsEmpty,
{
    #[allow(clippy::type_complexity)]
    fn lines(
        &self,
    ) -> Map<
        <Self as ItemsIndices<&'a str>>::ItemsIndices,
        impl FnMut((Self::Index, &'a str)) -> &'a str,
    > {
        <Self as ItemsIndices<&'a str>>::items(self)
    }
}
