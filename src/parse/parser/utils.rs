pub trait Reverse {
    fn reverse(self) -> Self;
}

// Useful to reverse the result of split_at.
impl<I> Reverse for (I, I) {
    fn reverse(self) -> Self {
        (self.1, self.0)
    }
}
