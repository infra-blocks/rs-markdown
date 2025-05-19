pub trait Parsable: Sized {
    type Quantity;

    // TODO: trim left, trim right, and split_at (the latter generates the two previous)
    /// Consumes a certain quantity within self and returns the remains.
    fn consume(self, quantity: Self::Quantity) -> Self;

    fn parsed<T>(self, quantity: Self::Quantity, value: T) -> ParseResult<Self, T> {
        let remaining = self.consume(quantity);
        Ok((remaining, value))
    }

    fn failed<T>(self) -> ParseResult<Self, T> {
        Err(self)
    }
}

pub type ParseResult<I, T> = Result<(I, T), I>;
