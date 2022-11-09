pub fn add_two(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_assert() {
        assert!(add_two(2, 2) == 4);
    }

    #[test]
    fn test_with_assert_eq() {
        assert_eq!(add_two(2, 2), 4);
    }

    #[test]
    fn test_with_assert_ne() {
        assert_ne!(add_two(2, 2), 3);
    }
}
