use super::{And, Map, ParseResult, Parser};

impl<I, T1, T2> Parser<I> for (T1, T2)
where
    I: Clone,
    T1: Parser<I>,
    T2: Parser<I>,
{
    type Output = (T1::Output, T2::Output);

    fn parse(&self, input: I) -> ParseResult<I, Self::Output> {
        let first = |input| self.0.parse(input);
        let second = |input| self.1.parse(input);

        first.and(second).parse(input)
    }
}

impl<I, T1, T2, T3> Parser<I> for (T1, T2, T3)
where
    I: Clone,
    T1: Parser<I>,
    T2: Parser<I>,
    T3: Parser<I>,
{
    type Output = (T1::Output, T2::Output, T3::Output);

    fn parse(&self, input: I) -> ParseResult<I, Self::Output> {
        let first = |input| self.0.parse(input);
        let second = |input| self.1.parse(input);
        let third = |input| self.2.parse(input);

        first
            .and(second)
            .and(third)
            .map(|((r1, r2), r3)| (r1, r2, r3))
            .parse(input)
    }
}

impl<I, T1, T2, T3, T4> Parser<I> for (T1, T2, T3, T4)
where
    I: Clone,
    T1: Parser<I>,
    T2: Parser<I>,
    T3: Parser<I>,
    T4: Parser<I>,
{
    type Output = (T1::Output, T2::Output, T3::Output, T4::Output);

    fn parse(&self, input: I) -> ParseResult<I, Self::Output> {
        let first = |input| self.0.parse(input);
        let second = |input| self.1.parse(input);
        let third = |input| self.2.parse(input);
        let fourth = |input| self.3.parse(input);

        first
            .and(second)
            .and(third)
            .and(fourth)
            .map(|(((r1, r2), r3), r4)| (r1, r2, r3, r4))
            .parse(input)
    }
}

impl<I, T1, T2, T3, T4, T5> Parser<I> for (T1, T2, T3, T4, T5)
where
    I: Clone,
    T1: Parser<I>,
    T2: Parser<I>,
    T3: Parser<I>,
    T4: Parser<I>,
    T5: Parser<I>,
{
    type Output = (T1::Output, T2::Output, T3::Output, T4::Output, T5::Output);

    fn parse(&self, input: I) -> ParseResult<I, Self::Output> {
        let first = |input| self.0.parse(input);
        let second = |input| self.1.parse(input);
        let third = |input| self.2.parse(input);
        let fourth = |input| self.3.parse(input);
        let fifth = |input| self.4.parse(input);

        first
            .and(second)
            .and(third)
            .and(fourth)
            .and(fifth)
            .map(|((((r1, r2), r3), r4), r5)| (r1, r2, r3, r4, r5))
            .parse(input)
    }
}

impl<I, T1, T2, T3, T4, T5, T6> Parser<I> for (T1, T2, T3, T4, T5, T6)
where
    I: Clone,
    T1: Parser<I>,
    T2: Parser<I>,
    T3: Parser<I>,
    T4: Parser<I>,
    T5: Parser<I>,
    T6: Parser<I>,
{
    type Output = (
        T1::Output,
        T2::Output,
        T3::Output,
        T4::Output,
        T5::Output,
        T6::Output,
    );

    fn parse(&self, input: I) -> ParseResult<I, Self::Output> {
        let first = |input| self.0.parse(input);
        let second = |input| self.1.parse(input);
        let third = |input| self.2.parse(input);
        let fourth = |input| self.3.parse(input);
        let fifth = |input| self.4.parse(input);
        let sixth = |input| self.5.parse(input);

        first
            .and(second)
            .and(third)
            .and(fourth)
            .and(fifth)
            .and(sixth)
            .map(|(((((r1, r2), r3), r4), r5), r6)| (r1, r2, r3, r4, r5, r6))
            .parse(input)
    }
}
