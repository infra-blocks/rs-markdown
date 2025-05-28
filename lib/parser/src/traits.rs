use std::iter::{Map, Peekable};

pub trait Parser<I> {
    type Output;
    fn parse(&self, input: I) -> ParseResult<I, Self::Output>;
}

impl<I, O, T> Parser<I> for T
where
    T: Fn(I) -> ParseResult<I, O>,
{
    type Output = O;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output> {
        self(input)
    }
}

pub trait Index: Copy {}

pub trait Indexable {
    type Index: Index;

    fn last_index(&self) -> Self::Index;
}

pub trait ItemsIndices<T>: Indexable {
    type ItemsIndices: Iterator<Item = (Self::Index, T)>;

    fn enumerate(&self) -> Enumerator<Self::ItemsIndices, Self::Index> {
        Enumerator::new(self.items_indices(), self.last_index())
    }
    fn items_indices(&self) -> Self::ItemsIndices;
    // The goal of returning the type here is to not coerce behind a trait implementation. However, it is hard (impossible?)
    // to specify the type returned here and alias it. So we just ask clippy to kindly stfu.
    #[allow(clippy::type_complexity)]
    fn items(&self) -> Map<Self::ItemsIndices, impl FnMut((Self::Index, T)) -> T> {
        self.items_indices().map(|(_, item)| item)
    }

    /// Returns the index that immediately follows the provided prefix.
    ///
    /// This function returns None if the prefix does not match items in sequence.
    fn after_prefix<U>(&self, prefix: U) -> Option<Self::Index>
    where
        U: IntoIterator<Item = T>,
        T: PartialEq,
    {
        let mut enumerator = self.enumerate();
        for item in prefix {
            match enumerator.next() {
                Some((_, i)) if i == item => {}
                _ => return None,
            }
        }
        Some(enumerator.next_index())
    }
}

pub struct Enumerator<I: Iterator, Index> {
    iter: Peekable<I>,
    last_index: Index,
}

impl<I, Idx> Enumerator<I, Idx>
where
    I: Iterator,
    Idx: Index,
{
    pub fn new(iter: I, last_index: Idx) -> Self {
        Self {
            iter: iter.peekable(),
            last_index,
        }
    }

    pub fn next_index<T>(&mut self) -> Idx
    where
        I: Iterator<Item = (Idx, T)>,
    {
        match self.iter.peek() {
            Some((index, _)) => *index,
            None => self.last_index,
        }
    }
}

impl<I, Index, T> Iterator for Enumerator<I, Index>
where
    I: Iterator<Item = (Index, T)>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub trait SplitAt: Indexable + Sized {
    fn split_at(&self, index: Self::Index) -> (Self, Self);
}

pub trait SubsetRange<T>: Indexable {
    /// Get the index of the element.
    fn subset_range(&self, item: T) -> (Self::Index, Self::Index);
}

pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

pub type ParseResult<I, T> = Result<(I, T), I>;
