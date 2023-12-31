use super::Solution;

impl Solution {
    /// Solution for Leetcode problem #20: [Valid Parenthesis](https://leetcode.com/problems/valid-parentheses/)
    ///
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
    ///
    /// # Approach #2
    /// The basic idea works, but we need to be more 'stack-like'.
    /// When we encounter an open bracket, we need to find the closing bracket,
    /// and then 'zoom in' on the contents inside, return true only if we can
    /// resolve every bracket.
    ///
    /// When we encounter an open bracket, it is added to the stack.
    /// It is removed from the stack if we find the corresponding closed bracket.
    ///
    /// Every open bracket encounterd is added to the stack.
    ///
    /// If we encounter a closing brakcet that doesn't match the current open bracket,
    /// we immedietly return false.
    ///
    /// If we encounter a matching closing bracket, we remove that bracket from the stack.
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        let s_vec: Vec<char> = s.chars().collect();
        let mut i = 0;

        while i < s_vec.len() {
            let c = Solution::open(s_vec[i]);
            match c {
                Some(c) => {
                    stack.push(c);
                }
                None => {
                    if stack.len() > 0 && stack[stack.len() - 1] == s_vec[i] {
                        stack.pop();
                    } else {
                        return false;
                    }
                }
            }
            i += 1;
        }
        return stack.len() == 0;
    }

    /// Checks if the char is an open bracket, or if closed, nothing
    fn open(c: char) -> Option<char> {
        match c {
            '(' => Some(')'),
            '{' => Some('}'),
            '[' => Some(']'),
            _ => None,
        }
    }
}

#[test]
fn example_2() {
    let s = format!("()[]{{}}");
    assert_eq!(Solution::is_valid(s), true);
}

#[test]
fn example_3() {
    let s = format!("(]");
    assert_eq!(Solution::is_valid(s), false);
}
