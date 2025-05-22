/// Returns a predicate that is true when the value equals the provided one.
pub fn is<T: PartialEq>(value: T) -> impl Fn(T) -> bool {
    move |i| i == value
}

/// Returns a predicate that is true when the value matches one of the provided ones.
///
/// The reverse of [is_none_of].
pub fn is_one_of<T: PartialEq>(values: &[T]) -> impl Fn(T) -> bool {
    move |i| values.contains(&i)
}

#[cfg(test)]
mod test {
    use super::*;

    mod is {
        use super::*;

        #[test]
        fn should_return_true_when_value_equals() {
            let predicate = is(1);
            assert!(predicate(1));
        }

        #[test]
        fn should_return_false_when_value_does_not_equal() {
            let predicate = is(1);
            assert!(!predicate(2));
        }

        #[test]
        fn should_work_with_str() {
            let predicate = is("a");
            assert!(predicate("a"));
            assert!(!predicate("b"));
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
}
