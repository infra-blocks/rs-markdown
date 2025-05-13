// TODO: figure out a way to minimize code duplication. Is there a way to generate the relevant functions with
// an opening and a closing symbol?
mod double_quotes;
mod parentheses;
mod single_quotes;

pub use double_quotes::*;
pub use parentheses::*;
pub use single_quotes::*;
