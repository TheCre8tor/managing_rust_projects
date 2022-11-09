pub fn add_two(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_assert() {
        let result = add_two(2, 2);
        let expected = 4;

        assert!(
            result == expected,
            "Expected {}; result was {}",
            expected,
            result
        );
    }

    #[test]
    fn test_with_assert_eq() {
        let result = add_two(2, 2);
        let expected = 4;

        assert_eq!(
            result, expected,
            "Expected {}; result was {}",
            expected, result
        );
    }

    #[test]
    fn test_with_assert_ne() {
        assert_ne!(add_two(2, 2), 3);
    }
}
