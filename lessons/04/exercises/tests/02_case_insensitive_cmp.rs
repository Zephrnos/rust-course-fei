//! Run this file with `cargo test --test 02_case_insensitive_cmp`.

//! TODO: Implement a struct `CaseInsensitive`, which will allow comparing (=, <, >, etc.)
//! two (ASCII) string slices in a case insensitive way, without performing any reallocations
//! and without modifying the original strings.

use std::cmp::Ordering;

struct CaseInsensitive<'a>(&'a str);

impl PartialEq for CaseInsensitive<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.0.len() == other.0.len() &&
        self.0.chars().zip(other.0.chars()).all(|(a, b)| 
            a.to_ascii_lowercase() == b.to_ascii_lowercase()
        )
    }
}

impl PartialOrd for CaseInsensitive<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_chars = self.0.chars().map(|c| c.to_ascii_lowercase());
        let other_chars = other.0.chars().map(|c| c.to_ascii_lowercase());
        
        for (a, b) in self_chars.zip(other_chars) {
            match a.cmp(&b) {
                Ordering::Equal => continue,
                other => return Some(other),
            }
        }
        
        // If all characters match, compare lengths
        Some(self.0.len().cmp(&other.0.len()))
    }
}


/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use crate::CaseInsensitive;

    #[test]
    fn case_insensitive_same() {
        assert!(CaseInsensitive("") == CaseInsensitive(""));
        assert!(CaseInsensitive("a") == CaseInsensitive("A"));
        assert!(CaseInsensitive("a") == CaseInsensitive("a"));
        assert!(CaseInsensitive("Foo") == CaseInsensitive(&String::from("fOo")));
        assert!(CaseInsensitive("12ABBBcLPQusdaweliAS2") == CaseInsensitive("12AbbbclpQUSdawelias2"));
    }

    #[test]
    fn case_insensitive_smaller() {
        assert!(CaseInsensitive("") < CaseInsensitive("a"));
        assert!(CaseInsensitive("a") < CaseInsensitive("B"));
        assert!(CaseInsensitive("aZa") < CaseInsensitive("Zac"));
        assert!(CaseInsensitive("aZ") < CaseInsensitive("Zac"));
        assert!(CaseInsensitive("PWEasUDsx") < CaseInsensitive("PWEaszDsx"));
        assert!(CaseInsensitive("PWEasuDsx") < CaseInsensitive("PWEasZDsx"));
    }

    #[test]
    fn case_insensitive_larger() {
        assert!(CaseInsensitive("a") > CaseInsensitive(""));
        assert!(CaseInsensitive("B") > CaseInsensitive("a"));
        assert!(CaseInsensitive("Zac") > CaseInsensitive("aZa"));
        assert!(CaseInsensitive("Zac") > CaseInsensitive("aZ"));
        assert!(CaseInsensitive("PWEaszDsx") > CaseInsensitive("PWEasUDsx"));
        assert!(CaseInsensitive("PWEasZDsx") > CaseInsensitive("PWEasuDsx"));
    }
}
