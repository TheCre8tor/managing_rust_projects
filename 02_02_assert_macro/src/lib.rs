pub fn is_even(number: i32) -> bool {
    match number % 2 {
        0 => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::is_even;

    #[test]
    fn test_is_even() {
        assert!(is_even(42));
        assert!(!is_even(13));
    }
}
