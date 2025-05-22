pub trait Reverse {
    fn reverse(self) -> Self;
}

// Useful to reverse the result of split_at.
impl<I> Reverse for (I, I) {
    fn reverse(self) -> Self {
        (self.1, self.0)
    }
}

// TODO: in its own module?
#[cfg(test)]
macro_rules! alias {
    ($alias:ident, $expression:expr) => {
        macro_rules! $alias {
            () => {
                $expression
            };
        }
    };
}
#[cfg(test)]
pub(super) use alias;
