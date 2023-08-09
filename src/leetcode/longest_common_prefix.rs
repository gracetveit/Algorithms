use super::Solution;

impl Solution {
    /// Solution for [Leetcode Problem #14: Longest Common Prefix](https://leetcode.com/problems/longest-common-prefix/)
    ///
    /// Finds the longest common prefix string among an array of strings. If there
    /// is no common prefix, return an empty string
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        todo!()
    }
}

#[test]
fn example_1(){
    let strs = vec![format!("flower"), format!("flow"), format!("flight")];
    assert_eq!(Solution::longest_common_prefix(strs), "fl");
}

#[test]
fn example_2(){
    let strs = vec![format!("dog"), format!("racecar"), format!("car")];
    assert_eq!(Solution::longest_common_prefix(strs), "");
}
