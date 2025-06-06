use parser::ParseResult;
use std::{fmt::Debug, result};

pub trait IBlock<'a> {
    type Open: IOpenBlock<'a>;

    fn open(line: &'a str) -> ParseResult<&'a str, Self::Open>;
}

#[cfg(test)]
pub trait IBlockTestExt<'a>: IBlock<'a> {
    fn strict_open(line: &'a str) -> Self::Open {
        match Self::open(line) {
            Ok((_, open)) => open,
            Err(rejected) => panic!("error strict opening block: {:?}", rejected),
        }
    }

    fn open_and_commit_all(lines: &'a str) -> Self::Open
    where
        Self::Open: Debug,
    {
        let mut lines = lines.split_inclusive("\n");
        let mut block = Self::strict_open(lines.next().unwrap());
        for line in lines {
            block
                .stage(line)
                .expect(format!("error staging line {} for block {:?}", line, block).as_str());
            block.commit();
        }
        block
    }

    fn open_and_close(lines: &'a str) -> <Self::Open as IOpenBlock<'a>>::Closed
    where
        Self::Open: Debug,
    {
        Self::open_and_commit_all(lines).close_and_return()
    }
}

#[cfg(test)]
impl<'a, T> IBlockTestExt<'a> for T where T: IBlock<'a> {}

pub trait IOpenBlock<'a>: Sized {
    type Closed;

    fn stage(&mut self, line: &'a str) -> Result<&'a str, ()>;
    fn commit(&mut self);
    fn close<F: FnMut(Self::Closed) -> ()>(self, sink: F);
}

#[cfg(test)]
pub trait IOpenBlockTestExt<'a>: IOpenBlock<'a> {
    fn close_and_return(self) -> Self::Closed {
        let mut result = None;
        self.close(|closed| result = Some(closed));
        result.expect("error closing block")
    }
}

#[cfg(test)]
impl<'a, T> IOpenBlockTestExt<'a> for T where T: IOpenBlock<'a> {}

/// Marker trait for blocks that consist of a single segment.
pub trait SingleSegmentBlock<'a> {
    type Closed;
}

/// Blanket implementation for blocks that consist of a single segment.
///
/// Requires the type to also implement `Debug` and `Into<Block<'a>>`.
impl<'a, T> IOpenBlock<'a> for T
where
    T: SingleSegmentBlock<'a> + Debug + Into<<Self as SingleSegmentBlock<'a>>::Closed>,
{
    type Closed = <Self as SingleSegmentBlock<'a>>::Closed;

    fn stage(&mut self, line: &'a str) -> Result<&'a str, ()> {
        // This always fails because we expect the single segment block to already be fully formed.
        Err(())
    }

    fn commit(&mut self) {
        panic!("called commit on a single segment block: {:?}", self);
    }

    fn close<F: FnMut(Self::Closed) -> ()>(self, mut sink: F) {
        sink(self.into());
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Staging<T> {
    staged: Option<T>,
}

impl<T> Default for Staging<T> {
    fn default() -> Self {
        Self { staged: None }
    }
}

impl<T> Staging<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, staged: T) {
        if self.staged.is_some() {
            panic!("staging already set");
        }
        self.staged = Some(staged);
    }

    pub fn commit<F: FnOnce(T) -> ()>(&mut self, sink: F) {
        sink(self.staged.take().expect("cannot commit empty staging"))
    }

    pub fn has_value_staged(&self) -> bool {
        self.staged.is_some()
    }

    pub fn reset(&mut self) {
        if self.staged.is_none() {
            panic!("cannot reset empty staging");
        }
        self.staged = None;
    }
}
