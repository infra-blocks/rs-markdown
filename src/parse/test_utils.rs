macro_rules! test_parse_macros {
    ($type:ty) => {
        macro_rules! failure_case {
            ($test:ident, $segment:expr) => {
                #[test]
                fn $test() {
                    use crate::parse::traits::Parse;

                    assert!(<$type>::parse::<nom::error::Error<&str>>($segment).is_err());
                }
            };
        }

        macro_rules! success_case {
                    ($test:ident, $segment:expr) => {
                        success_case!($test, $segment, $segment, "");
                    };
                    ($test:ident, $segment:expr, parsed => $parsed:expr) => {
                        success_case!($test, $segment, parsed => $parsed, "");
                    };
                    ($test:ident, $segment:expr, $parsed:expr, $remaining:expr) => {
                        success_case!($test, $segment, parsed => <$type>::new($parsed), $remaining);
                    };
                    ($test:ident, $segment:expr, parsed => $parsed:expr, $remaining:expr) => {
                        #[test]
                        fn $test() {
                            use crate::parse::traits::Parse;

                            assert_eq!(
                                <$type>::parse::<nom::error::Error<&str>>($segment),
                                Ok(($remaining, $parsed))
                            );
                        }
                    };
                }
    };
}

pub(super) use test_parse_macros;
