macro_rules! test_parse_macros {
    ($type:ty) => {
        macro_rules! failure_case {
            ($test:ident, $input:expr) => {
                #[test]
                fn $test() {
                    use crate::parse::traits::Parse;

                    let result = <$type>::parse($input);
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

                            assert_eq!(
                                <$type>::parse($input),
                                Ok(($remaining, $parsed))
                            );
                        }
                    };
                }
    };
}

pub(super) use test_parse_macros;
