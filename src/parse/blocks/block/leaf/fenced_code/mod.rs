mod backticks;
mod tildes;

use crate::parse::blocks::open_block::IBlock;
pub use backticks::*;
pub use tildes::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FencedCode<'a> {
    Backticks(BackticksFencedCode<'a>),
    Tildes(TildesFencedCode<'a>),
}

impl<'a> From<BackticksFencedCode<'a>> for FencedCode<'a> {
    fn from(fenced_code: BackticksFencedCode<'a>) -> Self {
        Self::Backticks(fenced_code)
    }
}

impl<'a> From<TildesFencedCode<'a>> for FencedCode<'a> {
    fn from(fenced_code: TildesFencedCode<'a>) -> Self {
        Self::Tildes(fenced_code)
    }
}

impl<'a> IBlock<'a> for FencedCode<'a> {
    type Open = open::FencedCode<'a>;

    fn open(line: &'a str) -> parser::ParseResult<&'a str, Self::Open> {
        if let Ok((remaining, open)) = BackticksFencedCode::open(line) {
            return Ok((remaining, open.into()));
        }
        if let Ok((remaining, open)) = TildesFencedCode::open(line) {
            return Ok((remaining, open.into()));
        }
        Err(line)
    }
}

pub mod open {
    use crate::parse::blocks::{
        block::leaf::fenced_code::{
            backticks::open::BackticksFencedCode, tildes::open::TildesFencedCode,
        },
        open_block::IOpenBlock,
    };

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum FencedCode<'a> {
        Backticks(BackticksFencedCode<'a>),
        Tildes(TildesFencedCode<'a>),
    }

    impl<'a> From<BackticksFencedCode<'a>> for FencedCode<'a> {
        fn from(fenced_code: BackticksFencedCode<'a>) -> Self {
            Self::Backticks(fenced_code)
        }
    }

    impl<'a> From<TildesFencedCode<'a>> for FencedCode<'a> {
        fn from(fenced_code: TildesFencedCode<'a>) -> Self {
            Self::Tildes(fenced_code)
        }
    }

    impl<'a> IOpenBlock<'a> for FencedCode<'a> {
        type Closed = super::FencedCode<'a>;

        fn stage(&mut self, line: &'a str) -> Result<&'a str, ()> {
            match self {
                Self::Backticks(block) => block.stage(line),
                Self::Tildes(block) => block.stage(line),
            }
        }

        fn commit(&mut self) {
            match self {
                Self::Backticks(block) => block.commit(),
                Self::Tildes(block) => block.commit(),
            }
        }

        fn close<F: FnMut(Self::Closed) -> ()>(self, mut sink: F) {
            match self {
                Self::Backticks(block) => block.close(|backticks| sink(backticks.into())),
                Self::Tildes(block) => block.close(|tildes| sink(tildes.into())),
            }
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;
        use crate::parse::blocks::open_block::IBlockTestExt;

        type Block<'a> = <FencedCode<'a> as IOpenBlock<'a>>::Closed;
        type Backticks<'a> = <BackticksFencedCode<'a> as IOpenBlock<'a>>::Closed;
        type Tildes<'a> = <TildesFencedCode<'a> as IOpenBlock<'a>>::Closed;

        #[test]
        fn shhould_work_with_backticks() {
            assert_eq!(
                Block::Backticks(Backticks::open_and_close("```rust\nlet x = 42;\n```\n")),
                Block::open_and_close("```rust\nlet x = 42;\n```\n")
            );
        }

        #[test]
        fn should_work_with_tildes() {
            assert_eq!(
                Block::Tildes(Tildes::open_and_close("~~~rust\nlet x = 42;\n~~~\n")),
                Block::open_and_close("~~~rust\nlet x = 42;\n~~~\n")
            );
        }
    }
}
