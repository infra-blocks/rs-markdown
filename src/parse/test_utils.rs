use super::traits::Parse;
use crate::parse::{Lines, parser::Enumerate};
use std::fmt::Debug;

macro_rules! test_parse_macros {
    ($type:ty) => {
        macro_rules! failure_case {
            ($test:ident, $input:expr) => {
                #[test]
                fn $test() {
                    use crate::parse::traits::Parse;
                    use crate::parse::Lines;

                    let result = <$type>::parse(Lines::from($input));
                    assert!(result.is_err(), "{:?}", result);
                }
            };
        }

        macro_rules! success_case {
                    ($test:ident, $input:expr) => {
                        success_case!($test, $input, $input, "");
                    };
                    ($test:ident, $input:expr, parsed => $parsed:expr) => {
                        success_case!($test, $input, parsed => $parsed, "");
                    };
                    ($test:ident, $input:expr, $parsed:expr, $remaining:expr) => {
                        success_case!($test, $input, parsed => <$type>::new($parsed), $remaining);
                    };
                    ($test:ident, $input:expr, parsed => $parsed:expr, $remaining:expr) => {
                        #[test]
                        fn $test() {
                            use crate::parse::traits::Parse;
                            use crate::parse::Lines;

                            assert_eq!(
                                <$type>::parse(Lines::from($input)),
                                Ok((Lines::from($remaining), $parsed))
                            );
                        }
                    };
                }
    };
}
pub(super) use test_parse_macros;

#[cfg(test)]
pub trait StrictParse<'a>
where
    Self: Sized + Debug,
{
    fn strict_parse(input: &'a str) -> Self;
}

#[cfg(test)]
impl<'a, U> StrictParse<'a> for U
where
    U: Parse<&'a str> + Debug,
{
    fn strict_parse(input: &'a str) -> Self {
        let (remaining, parsed) = Self::parse(Lines::from(input)).unwrap();
        assert!(
            remaining.is_empty(),
            "remaining input after strict parse: {remaining:?}"
        );
        parsed
    }
}
