//! Run this file with `cargo test --test 02_strip_prefix`.

// TODO: Implement a function called `strip_prefix`, which will take two strings (`needle` and `prefix`).
// It will return a substring of `needle` starting at the first character that is not contained
// in `prefix`.
// See tests for examples. Take a look at the `strip_prefix_lifetime_check` test!
//
// Hint: you can use `string.chars()` for iterating the Unicode characters of a string.

fn strip_prefix(needle: &str, prefix: &str) -> String {
    let prefix: Vec<_> = prefix.chars().collect();
    let split_pos = {
        // Position of the split
        &needle
        .chars()
        .position(|c| !prefix.contains(&c))
    };

    // match split_pos {
    //     Some(rposition) => {
    //         let mut needle_string = String::from(needle);
    //         needle_string.replace_range(..(rposition), "");
    //         needle_string
    //     },
    //     None => String::from("")
    // }

    match split_pos {
        Some(position) => {
            let mut needle_string = String::from(needle);
            let mut _garbage;
            for _ in 0..*position {
                _garbage = needle_string.remove(0);
            }
            needle_string
        },
        None => String::from("")
    }

}



/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use crate::strip_prefix;

    #[test]
    fn strip_prefix_basic() {
        assert_eq!(strip_prefix("foobar", "of"), "bar");
    }

    #[test]
    fn strip_prefix_full_result() {
        assert_eq!(strip_prefix("foobar", "x"), "foobar");
    }

    #[test]
    fn strip_prefix_empty_result() {
        assert_eq!(strip_prefix("foobar", "fbaro"), "");
    }

    #[test]
    fn strip_prefix_unicode() {
        assert_eq!(strip_prefix("čaukymňauky", "čaukym"), "ňauky");
    }

    #[test]
    fn strip_prefix_lifetime_check() {
        let needle = "foobar";
        let prefix = String::from("f");
        let result = strip_prefix(needle, &prefix);
        // TODO: Uncomment the `drop(prefix)` line.
        // Does the test still work? If not, fix `strip_prefix`!
        drop(prefix);
        assert_eq!(result, "oobar");
    }
}
