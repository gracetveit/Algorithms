use super::Solution;

impl Solution {
    /// Solution for [Leetcode Problem #14: Longest Common Prefix](https://leetcode.com/problems/longest-common-prefix/)
    ///
    /// Finds the longest common prefix string among an array of strings. If there
    /// is no common prefix, return an empty string
    ///
    /// # Example
    /// ```rust
    /// use algo::leetcode::Solution;
    ///
    /// let strs = vec![format!("flower"), format!("flow"), format!("flight")];
    /// assert_eq!(Solution::longest_common_prefix(strs), "fl");
    /// ```
    ///
    /// # Problem Solving
    /// The most obvious answer seems to be to index the first character of the
    /// first string, and check if that is equal to the remaining strings.
    /// If it is equal, add that to its own string, and repeat until there is
    /// a single mis-match. At that point, return the new string.
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut new_string: Vec<char> = Vec::new();
        let mut end_loop = false;
        let mut i = 0;

        while !end_loop {
            match strs[0].chars().nth(i) {
                None => end_loop = true,
                Some(primary_char) => {
                    for x in &strs {
                        match x.chars().nth(i) {
                            None => {
                                end_loop = true;
                                break;
                            }
                            Some(check_char) => {
                                if primary_char != check_char {
                                    end_loop = true;
                                    break;
                                }
                            }
                        }
                    }
                    if !end_loop {
                        new_string.push(primary_char);
                        i += 1;
                    }
                }
            }
        }

        return new_string.into_iter().collect();
    }
}

#[test]
fn example_1() {
    let strs = vec![format!("flower"), format!("flow"), format!("flight")];
    assert_eq!(Solution::longest_common_prefix(strs), "fl");
}

#[test]
fn example_2() {
    let strs = vec![format!("dog"), format!("racecar"), format!("car")];
    assert_eq!(Solution::longest_common_prefix(strs), "");
}
