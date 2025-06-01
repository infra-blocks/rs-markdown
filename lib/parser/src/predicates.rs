use crate::Parser;

/// Returns a predicate that is true when the value equals the provided one.
pub fn equals<T: PartialEq>(value: T) -> impl Fn(T) -> bool {
    move |i| i == value
}

/// Turns any parser into a predicate that returns whether the parse
/// result is successful.
pub fn is<P: Parser<I>, I>(parser: P) -> impl Fn(I) -> bool {
    move |i| parser.parse(i).is_ok()
}

/// Returns a predicate that is true when the value matches one of the provided ones.
///
/// The reverse of [is_none_of].
pub fn is_one_of<T: PartialEq>(values: &[T]) -> impl Fn(T) -> bool {
    move |i| values.contains(&i)
}

/// Returns a predicate that negates the return of the provided predicate.
pub fn not<F, I>(predicate: F) -> impl Fn(I) -> bool
where
    F: Fn(I) -> bool,
{
    move |i| !predicate(i)
}

#[cfg(test)]
mod test {
    use super::*;

    mod equals {
        use super::*;

        #[test]
        fn should_return_true_when_value_equals() {
            let predicate = equals(1);
            assert!(predicate(1));
        }

        #[test]
        fn should_return_false_when_value_does_not_equal() {
            let predicate = equals(1);
            assert!(!predicate(2));
        }

        #[test]
        fn should_work_with_str() {
            let predicate = equals("a");
            assert!(predicate("a"));
            assert!(!predicate("b"));
        }
    }

    mod is {
        use super::*;
        use crate::tag;

        #[test]
        fn should_return_true_when_parser_succeeds() {
            let predicate = is(tag("toto"));
            assert!(predicate("toto toto"));
        }

        #[test]
        fn should_return_false_when_parser_fails() {
            let predicate = is(tag("toto"));
            assert!(!predicate("tata"));
        }
    }

    mod is_one_of {
        use super::*;

        #[test]
        fn should_return_true_when_value_is_in_list() {
            let predicate = is_one_of(&[1, 2, 3]);
            assert!(predicate(1));
            assert!(predicate(2));
            assert!(predicate(3));
        }

        #[test]
        fn should_return_false_when_value_is_not_in_list() {
            let predicate = is_one_of(&[1, 2, 3]);
            assert!(!predicate(4));
            assert!(!predicate(5));
        }

        #[test]
        fn should_work_with_str() {
            let predicate = is_one_of(&["a", "b", "c"]);
            assert!(predicate("a"));
            assert!(predicate("b"));
            assert!(predicate("c"));
            assert!(!predicate("d"));
            assert!(!predicate("e"));
            assert!(!predicate("f"));
        }
    }

    mod not {
        use super::*;

        #[test]
        fn should_return_true_when_predicate_is_false() {
            let predicate = not(equals(1));
            assert!(predicate(2));
        }

        #[test]
        fn should_return_false_when_predicate_is_true() {
            let predicate = not(equals(1));
            assert!(!predicate(1));
        }

        #[test]
        fn should_work_with_str() {
            let predicate = not(equals("a"));
            assert!(predicate("b"));
            assert!(!predicate("a"));
        }
    }
}
