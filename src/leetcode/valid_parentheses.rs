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
    pub fn is_valid(s: String) -> bool {
        todo!()
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
