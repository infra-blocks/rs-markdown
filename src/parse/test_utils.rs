macro_rules! test_parse_macros {
    ($type:ty) => {
        macro_rules! failure_case {
            ($test:ident, $segment:expr) => {
                #[test]
                fn $test() {
                    use crate::parse::input::Parse;

                    assert!(
                        <$type>::parse(crate::parse::input::LinesInput::from($segment)).is_err()
                    )
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
                            use crate::parse::input::Parse;

                            let (remaining, parsed) =
                                <$type>::parse(crate::parse::input::LinesInput::from($segment))
                                    .expect("unexpected parse error");
                            assert_eq!($parsed, parsed);
                            assert_eq!($remaining, remaining.remaining());
                        }
                    };
                }
    };
}

pub(super) use test_parse_macros;
