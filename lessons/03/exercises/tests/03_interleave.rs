//! Run this file with `cargo test --test 03_interleave`.

// TODO: Implement a function called `interleave`, which will take two string slices and return
// a string that contains the characters from the two input strings interleaved.
// The first character of the output should start with the first character of the first argument.
// See tests for details.
//
// Can you write the function without any explicit indexing (`str[index]`)?
//
// Hint: you can use `string.chars()` to create an iterator over the Unicode characters of a string.

fn interleave(s1: &str, s2: &str) -> String {
    let mut s1 = s1.chars();
    let mut s2 = s2.chars();
    let mut return_string: String = "".to_owned();

    loop {
        match (s1.next(), s2.next()) {
            (Some(a), Some(b)) => {
                return_string += &a.to_string();
                return_string += &b.to_string();
            },
            (Some(a), None) => {return_string += &a.to_string()},
            (None, Some(a)) => {return_string += &a.to_string()},
            (None, None) => {break}
        }
    }

    return_string

}

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use crate::interleave;

    #[test]
    fn interleave_empty() {
        assert_eq!(interleave("", ""), "");
    }

    #[test]
    fn interleave_only_left() {
        assert_eq!(interleave("a", ""), "a");
        assert_eq!(interleave("zxjas", ""), "zxjas");
    }

    #[test]
    fn interleave_only_right() {
        assert_eq!(interleave("", "z"), "z");
        assert_eq!(interleave("", "foobar"), "foobar");
    }

    #[test]
    fn interleave_same_length() {
        assert_eq!(interleave("abcdef", "012345"), "a0b1c2d3e4f5");
    }

    #[test]
    fn interleave_first_longer() {
        assert_eq!(
            interleave("Programming Rust", "O'Reilly"),
            "POr'oRgerialmlmying Rust"
        );
    }

    #[test]
    fn interleave_second_longer() {
        assert_eq!(
            interleave("ahoj, jak se máš?", "díky za optání, mám se dobře"),
            "adhíokjy,  zjaa ko psteá nmíá,š ?mám se dobře"
        );
    }
}

// fn test_mem() {
//     let elem1: String = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_owned();
//     println!("{}", &elem1);
//     let elem1 = "abcdefghijklmnopqrstuvwxyz";
//     println!("{}", &elem1);
//     let elem2: String = "0123456789".to_owned();
//     println!("{}", &elem2);
// }

// #[cfg(test)]

// mod tests {
//     use crate::test_mem;

//     #[test]
//     fn interleave_empty() {
//         test_mem();
//     }

// }
