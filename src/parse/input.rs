use parser::{IsEmpty, ItemsIndices, SplitAt, SubsetRange};
use std::fmt::Debug;

/// A trait to regroup different all the different requirements to use all parser functionalities.
pub trait Input<T>
where
    Self: ItemsIndices<T> + SubsetRange<T> + SplitAt + Clone + Debug + IsEmpty,
{
}
