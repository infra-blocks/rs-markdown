use parser::ParseResult;
use std::fmt::Debug;

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
    fn strict_open_and_commit_all(lines: &'a str) -> Self::Open
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
}

#[cfg(test)]
impl<'a, T> IBlockTestExt<'a> for T where T: IBlock<'a> {}

pub trait IOpenBlock<'a> {
    type Closed;

    fn stage(&mut self, line: &'a str) -> Result<&'a str, ()>;
    fn commit(&mut self);
    fn close(self) -> Self::Closed;
}

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

    fn close(self) -> Self::Closed {
        self.into()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Staging<T> {
    staged: Option<T>,
}

impl<T> Staging<T> {
    pub fn new() -> Self
    where
        T: Default,
    {
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
