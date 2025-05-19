use std::iter::Peekable;

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

pub trait Enumerate<T>: Indexable {
    fn enumerate(&self) -> impl Enumerator<Self::Index, T> {
        EnumeratorImpl::new(self.items_indices(), self.last_index())
    }
    fn items_indices(&self) -> impl Iterator<Item = (Self::Index, T)>;
    fn is_empty(&self) -> bool {
        self.items_indices().next().is_none()
    }
}

pub trait Enumerator<I, T>: Iterator<Item = (I, T)>
where
    I: Index,
{
    fn next_index(&mut self) -> I;
}

struct EnumeratorImpl<I: Iterator, Index> {
    iter: Peekable<I>,
    last_index: Index,
}

impl<I, Index> EnumeratorImpl<I, Index>
where
    I: Iterator,
{
    pub fn new(iter: I, last_index: Index) -> Self {
        Self {
            iter: iter.peekable(),
            last_index,
        }
    }
}

impl<I, L, T> Enumerator<L, T> for EnumeratorImpl<I, L>
where
    L: Index,
    I: Iterator<Item = (L, T)>,
{
    fn next_index(&mut self) -> L {
        match self.iter.peek() {
            Some((index, _)) => *index,
            None => self.last_index,
        }
    }
}

impl<I, Index, T> Iterator for EnumeratorImpl<I, Index>
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

pub type ParseResult<I, T> = Result<(I, T), I>;
