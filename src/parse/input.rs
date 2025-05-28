use parser::{IsEmpty, ItemsIndices, SplitAt, SubsetRange};
use std::fmt::Debug;

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
}
