//! Run this file with `cargo test --test 06_write_tests`.

/// This function implements a string sanitization logic that should uphold the following
/// properties:
/// - After sanitization, the result must not end with the character `x`
/// - After sanitization, the result must not end with the character `o`
/// - After sanitization, the result must not end with the string `.exe`
///
/// The function assumes that the input to the function only consists of lower and uppercase
/// characters from the English alphabet and digits 0-9.
///
/// The implementation contains some bugs.
///
/// Your task is to write a set (at least 8) of unit tests, use them to find (at least 2) bugs in
/// this function and then fix the function.
fn sanitize(input: &str) -> &str {

    let mut input  = input;
    if input.ends_with(".exe") {
        input = &input[0..input.len() - 4];
    }

    // Remove all x from the end of the string
    input = input.trim_end_matches("x");
    // Remove all o from the end of the string
    input = input.trim_end_matches("o");

    

    input
}

/// TODO: write tests for the `sanitize` function
///
/// Bonus: can you find any bugs using the [proptest](https://proptest-rs.github.io/proptest/intro.html)
/// crate?
/// Note that you will probably need to run `cargo test` with the `PROPTEST_DISABLE_FAILURE_PERSISTENCE=1`
/// environment variable to make proptest work for tests stored in the `tests` directory.
#[cfg(test)]
mod tests {
    use super::sanitize;

    #[test]
    fn test_x() {
        let input_str: &str = "sex";
        assert_eq!(sanitize(&input_str), "se")
    }

    #[test]
    fn test_o() {
        let input_str: &str = "to";
        assert_eq!(sanitize(&input_str), "t")
    }

    #[test]
    fn test_in_x() {
        let input_str: &str = "text";
        assert_eq!(sanitize(&input_str), "text")
    }

    #[test]
    fn test_in_o() {
        let input_str: &str = "top";
        assert_eq!(sanitize(&input_str), "top")
    }

    #[test]
    fn test_multi_x() {
        let input_str: &str = "franxx";
        assert_eq!(sanitize(&input_str), "fran")
    }

    #[test]
    fn test_multi_o() {
        let input_str: &str = "too";
        assert_eq!(sanitize(&input_str), "t")
    }

    #[test]
    fn test_no_exe_no_x() {
        let input_str: &str = "sex.exe";
        assert_eq!(sanitize(&input_str), "se")
    }

    #[test]
    fn test_x_and_o() {
        let input_str: &str = "Mox";
        assert_eq!(sanitize(&input_str), "M")
    }

    #[test]
    fn test_no_exe_no_o() {
        let input_str: &str = "goo.exe";
        assert_eq!(sanitize(&input_str), "g")
    }

}
