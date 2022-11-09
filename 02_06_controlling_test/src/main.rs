fn is_even(n: i32) -> bool {
    println!("Evaluating if {} is even...", n);
    match n % 2 {
        0 => true,  // even
        _ => false, // odd
    }
}

/* PASSING ARGUMENTS TO cargo test
*
* cargo test [arguments] -- [arguments]
*
* arguments passed before the hyphen seperator
* are passed to the cargo test utility.
*
* arguments that come after the seperator go
* to the test binary itself
*

? cargo test --help
  - Display options for use with cargo test

? cargo test -- --help
  - Display options for use with the test binary

? cargo test -- --show-output
  - if we want to see everything that's printed to the standard output

? cargo test [test_name]
  - run a single test

* SEQUENTIAL TEST EXECUTION ->
? cargo test -- --test-threads=1

? cargo test -- --ignored
  - run the tests with #[ignore]

* */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        assert_eq!(is_even(0), true);
    }

    #[test]
    fn test_positive_numbers() {
        assert_eq!(is_even(1), true); // This is wrong
        assert_eq!(is_even(2), true);
        assert_eq!(is_even(3), false);
    }

    #[test]
    #[ignore]
    fn test_negative_numbers() {
        assert_eq!(is_even(-1), false);
        assert_eq!(is_even(-2), true);
        assert_eq!(is_even(-3), false);
    }
}
