use super::Solution;

impl Solution {
    /// Given a string `s` containing just the characters `'('`, `')'`, `'{'`,
    /// `'}'`, `'['`, and `']'`, determin if the input string is valid. A string
    /// is valid if:
    /// 1. Open brackets must be closed by the same type of brackets
    /// 2. Open brackets must be closed in the correct order.
    /// 3. Every close bracket has a corresponding open bracket of the same type.
    ///
    /// # Example
    /// ```rust
    /// use algo::leetcode::Solution;
    ///
    /// let s = format!("()");
    /// assert_eq!(Solution::is_valid(s), true);
    /// ```
    ///
    /// # Approach
    /// Looking to use a stack-based solution. Converts the string into a vec
    /// of chars, and then iterates through the vec, 'popping' the first element.
    /// If it's a closed bracket, immedietly return false.
    /// If it's an open bracket, go through each remaining element of the vec.
    /// If you find a matching element, remove it, and stop matching, moving
    /// onto the nth + 1 element.
    /// If you reach the end and can't find a matching element, return false.
    /// If the vec is then empty, return true.
    pub fn is_valid(s: String) -> bool {
        if s.len() == 0 {
            return true
        }

        // Turn into vec
        let mut s_vec: Vec<char> = s.chars().collect();
        let checked_char = s_vec.remove(0);
        // Check first Element
        match Solution::open(checked_char) {
            Some(c) => {
                // Iterate through vec
                match Solution::closed(c, s_vec) {
                    Some(updated_vec) => {
                        let new_string: String = updated_vec.into_iter().collect();
                        return Solution::is_valid(new_string);
                    }
                    None => return false
                }
            }
            None => return false
        }
    }

    /// Checks if the char is an open bracket, or if closed, nothing
    pub fn open(c: char) -> Option<char> {
        match c {
            '(' => Some(')'),
            '{' => Some('}'),
            '[' => Some(']'),
            _ => None
        }
    }

    /// Checks if the vec contains the matching closing character.
    /// Returns none if no such matching character exists; otherwise returns
    /// an updated vec with the closing char removed.
    pub fn closed(c: char, mut v: Vec<char>) -> Option<Vec<char>> {
        let mut i = 0;
        while i < v.len() {
            if v[i] == c {
                v.remove(i);
                return Some(v);
            }
            i += 1;
        }
        return None;
    }
}

#[test]
fn example_2(){
    let s = format!("()[]{{}}");
    assert_eq!(Solution::is_valid(s), true);
}

#[test]
fn example_3(){
    let s = format!("(]");
    assert_eq!(Solution::is_valid(s), false);
}
