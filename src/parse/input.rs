use super::parser::{Enumerate, Indexable, SplitAt};
use std::fmt::Debug;

pub trait IndexOf<T>: Indexable {
    /// Get the index of the element.
    fn index_of(&self, item: T) -> Self::Index;
}

/// A trait to regroup different all the different requirements to use all parser functionalities.
pub trait Input<T>
where
    Self: Enumerate<T> + IndexOf<T> + SplitAt + Clone + Debug,
{
}
