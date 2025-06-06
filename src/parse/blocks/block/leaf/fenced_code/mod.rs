mod backticks;
mod tildes;

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
