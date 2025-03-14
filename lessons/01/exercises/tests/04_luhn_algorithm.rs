//! Run this file with `cargo test --test 04_luhn_algorithm`.

// TODO: Implement the Luhn algorithm (https://en.wikipedia.org/wiki/Luhn_algorithm),
// which is used to check the validity of e.g. bank or credit card numbers.

fn num_to_digits(num: i64) -> Vec<i32> {
    num.to_string() // Convert the number to a string
        .chars() // Get an iterator over the characters
        .map(|c| c.to_digit(10).unwrap() as i32) // Convert each character back to a digit
        .collect() // Collect the digits into a vector
}

fn luhn_algorithm(num: i64) -> bool {
    let mut digits = num_to_digits(num);
    match digits.pop() {
        None => panic!("no check digit found"),
        Some(check_digit) => {
            // Reverse the digits to process from right to left
            digits.reverse();

            let mut total: i32 = 0;
            for (i, elem) in digits.iter().enumerate() {
                if i % 2 == 0 {
                    // Double every second digit (0-based index after reversing)
                    let doubled = elem * 2;
                    total += if doubled > 9 { doubled - 9 } else { doubled };
                } else {
                    total += elem;
                }
            }

            // Calculate the check digit
            total = (10 - (total % 10)) % 10;
            total == check_digit
        }
    }
}

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use super::luhn_algorithm;

    #[test]
    fn luhn_zero() {
        assert!(luhn_algorithm(0));
    }

    #[test]
    fn luhn_small_correct() {
        assert!(luhn_algorithm(18));
    }

    #[test]
    fn luhn_small_incorrect() {
        assert!(!luhn_algorithm(10));
    }

    #[test]
    fn luhn_correct() {
        assert!(luhn_algorithm(17893729974));
        assert!(luhn_algorithm(79927398713));
    }

    #[test]
    fn luhn_incorrect() {
        assert!(!luhn_algorithm(17893729975));
        assert!(!luhn_algorithm(17893729976));
        assert!(!luhn_algorithm(17893729977));
        assert!(!luhn_algorithm(123456));
    }
}
