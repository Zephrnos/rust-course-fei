//! Run this file with `cargo test --test 05_match_parentheses`.

// TODO: Implement a function called `match_parentheses`.
// It will receive a string slice (`&str`) containing arbitrary characters.
//
// Check that all parentheses within the string (`()`, `[]`, `{}`) are matched properly:
// - Each `(` has to be before `)`, `[` before `]` and `{` before `}`
// - There is an even number of opening and closing parentheses of each kind
// - Parentheses are not mismatched: e.g. `(` cannot be "closed" by `]`
//
// If everything is matched properly, return `true`, otherwise return `false`.
//
// Try to leverage pattern matching.
// Hint: there is a useful basic data structure that can be used for checking parentheses equality.
// It rhymes with "Jack" :)
// You don't even need to implement it fully, just use Vec and perform two operations on it.

use std::collections::HashMap;

fn match_parentheses(string_slice: &str) -> bool {
    // Mapping open and closed parenthesis
    let brackets: [char; 6] = ['(', ')', '[', ']', '{', '}'];
    let mapped_brackets: HashMap<char, char> = HashMap::from([
        (')', '('),
        ('}', '{'),
        (']', '['),
    ]);
    // Create Simple Stack
    let mut stack = Vec::new();
    // Loop over the string
    for elem in string_slice.chars(){
        // Check if char is a bracket
        match brackets.contains(&elem){
            false => {},
            true => {
                // Check if stack is empty or not
                match stack.pop(){
                // If empty, push element
                None => stack.push(elem),
                // If not, attempt to match the brackets
                Some(top_elem) => {
                    // Match to a closing bracket
                    match &mapped_brackets.get(&elem){
                        // If its not a closing bracket, push it onto the stack along with the previously popped element
                        None => {
                            stack.push(top_elem);
                            stack.push(elem);
                        },
                        // If its not a matching bracket type, push both brackets back onto the stack. Else, do nothing.
                        Some(open_bracket) => {
                            if *open_bracket != &top_elem{
                                stack.push(top_elem);
                                stack.push(elem);
                                }
                            }
                        }
                    }
                }
            }
        }
        
    }
    match stack.pop(){
        None => return true,
        Some(_) => return false
    }
}

fn match_parentheses(string_slice: &str) -> bool {
    let pairs: HashMap<_, _> = [('(', ')'), ('{', '}'), ('[', ']')].into();
    let closers: Vec<char> = pairs.values().copied().collect();

    let mut stack = vec![];

    for c in string_slice.chars() {
        let is_closing_char = closers.contains(&c);

        if is_closing_char {
            // if we are waiting for this closing char, pop it off the stack
            if stack.last() == Some(&c) {
                stack.pop();
            } 
            // encountering a closing charater we aren't waiting for is immediate failure
            else {
                return false;
            }
        } 
        // if this is the start of a pair, push the expected close onto the stack
        else if let Some(closing_char) = pairs.get(&c) {
            stack.push(*closing_char)
        }
    }

    // if we are still waiting for anything, we failed
    stack.is_empty()
}

fn match_parentheses(string_slice: &str) -> bool {
    let pairs: HashMap<_, _> = [('(', ')'), ('{', '}'), ('[', ']')].into();
    let closers: Vec<char> = pairs.values().copied().collect();

    let mut stack = vec![];

    for c in string_slice.chars() {
        let is_closing_char = closers.contains(&c);
        let is_opening_char = pairs.get(&c);
        let waiting_for = stack.last();

        match (is_closing_char, is_opening_char, waiting_for) {
            (false, None, _) => (),
            (false, Some(closing_char), _) => stack.push(*closing_char),
            (true, None, None) => return false,
            (true, None, Some(waiting_for)) => {
                if c == *waiting_for {
                    stack.pop();
                } else {
                    return false;
                }
            }
            (true, Some(_), _) => panic!("{c} is defined as both an opening and closing char"),
        }
    }

    stack.is_empty()
}

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use super::match_parentheses;

        #[test]
    fn simple() {
        assert!(match_parentheses("[]"));
        assert!(match_parentheses("()"));
        assert!(match_parentheses("{}"));
        assert!(match_parentheses("{}{}"));
        assert!(match_parentheses("{}{}{}"));
        assert!(match_parentheses("{}[]()"));
        assert!(match_parentheses("y{x}qq(xywe)[][xx]yy[][p]()"));
    }

    #[test]
    fn nested() {
        assert!(match_parentheses("({[]})"));
        assert!(match_parentheses("q(x{x[xqp]yy}y)"));
        assert!(match_parentheses("((((()))))"));
        assert!(match_parentheses("{[({})](([]))}"));
    }

    #[test]
    fn match_parentheses_empty() {
        assert!(match_parentheses(""));
        assert!(match_parentheses("foobar"));
    }

    #[test]
    fn match_parentheses_wrong_order() {
        assert!(!match_parentheses(")("));
        assert!(!match_parentheses("xx)y(aa"));
    }

    #[test]
    fn match_parentheses_extra_opening() {
        assert!(!match_parentheses("("));
        assert!(!match_parentheses("x[qq]e(s"));
        assert!(!match_parentheses("[(]"));
        assert!(!match_parentheses("(xxú[ú]😊"));
    }

    #[test]
    fn match_parentheses_extra_closing() {
        assert!(!match_parentheses(")"));
        assert!(!match_parentheses("[])"));
        assert!(!match_parentheses("[)]"));
        assert!(!match_parentheses("x([{)}])y"));
    }

    #[test]
    fn match_parentheses_wrong_matched_type() {
        assert!(!match_parentheses("[)"));
        assert!(!match_parentheses("[qp)"));
        assert!(!match_parentheses("[}xx"));
        assert!(!match_parentheses("p{]vr"));
        assert!(!match_parentheses("y[q}xx"));
        assert!(!match_parentheses("y[qq)x"));
        assert!(!match_parentheses("([})"));
        assert!(!match_parentheses("(((([}))))"));
    }

    #[test]
    fn respect_relative_ordering() {
        assert!(!match_parentheses("([)]"));
    }

}
